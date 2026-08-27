document.addEventListener("DOMContentLoaded", () => {
    if (!window.searchIndex || typeof elasticlunr === "undefined") {
        console.error("Search index or Elasticlunr is not available.");
        return;
    }

    let searchGeneration = 0;

    const index = elasticlunr.Index.load(window.searchIndex);

    const headerForm = document.getElementById("header-search-form");
    const headerInput = document.getElementById("header-search-input");
    const headerResults = document.getElementById("header-search-results");

    const pageForm = document.getElementById("search-form");
    const pageInput = document.getElementById("search-input");
    const pageResults = document.getElementById("search-results");

    if (headerForm && headerInput && headerResults) {
        setupHeaderSearch(
            index,
            headerForm,
            headerInput,
            headerResults
        );
    }

    if (pageForm && pageInput && pageResults) {
        setupPageSearch(
            index,
            pageForm,
            pageInput,
            pageResults
        );
    }
});


function search(index, query) {
    return index.search(query, {
        fields: {
            title: {
                boost: 2
            },
            body: {
                boost: 1
            }
        },
        bool: "OR",
        expand: true
    });
}


function setupHeaderSearch(index, form, input, container) {
    let activeIndex = -1;
    input.addEventListener("input", async () => {
        const query = input.value.trim();

        activeIndex = -1;

        if (!query) {
            closeResults(container);
            return;
        }

        const generation = ++searchGeneration;

        const results = search(index, query).slice(0, 5);

        await displayHeaderResults(
            results,
            query,
            container,
            generation,
            () => searchGeneration
        );
    });


    form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const query = input.value.trim();

        if (!query) {
            return;
        }

        const results = search(index, query);

        if (results.length === 0) {
            return;
        }

        window.location.href = normalizeUrl(results[0].ref);
    });


    input.addEventListener("keydown", (event) => {
        const items = container.querySelectorAll(".search-result");

        if (!items.length) {
            if (event.key === "Escape") {
                closeResults(container);
                input.blur();
            }

            return;
        }


        if (event.key === "ArrowDown") {
            event.preventDefault();

            activeIndex = Math.min(
                activeIndex + 1,
                items.length - 1
            );

            updateActiveResult(items, activeIndex);
        }


        if (event.key === "ArrowUp") {
            event.preventDefault();

            activeIndex = Math.max(
                activeIndex - 1,
                0
            );

            updateActiveResult(items, activeIndex);
        }


        if (event.key === "Enter") {
            event.preventDefault();

            if (activeIndex >= 0) {
                items[activeIndex].click();
                return;
            }

            items[0].click();
        }


        if (event.key === "Escape") {
            closeResults(container);
            input.blur();
        }
    });


    document.addEventListener("click", (event) => {
        if (!form.contains(event.target)) {
            closeResults(container);
        }
    });
}


async function displayHeaderResults(
    results,
    query,
    container,
    generation,
    getCurrentGeneration
) {
    container.innerHTML = "";

    if (results.length === 0) {
        container.innerHTML = `
            <div class="search-no-results">
                No results found.
            </div>
        `;

        container.hidden = false;
        return;
    }

    for (const result of results) {
        const link = document.createElement("a");

        link.href = normalizeUrl(result.ref);
        link.className = "search-result";

        const title = document.createElement("div");
        title.className = "search-result-title";

        const excerpt = document.createElement("div");
        excerpt.className = "search-result-excerpt";

        try {
            const response = await fetch(result.ref);
            const html = await response.text();

            if (generation !== getCurrentGeneration()) {
                return;
            }

            const documentPage = new DOMParser()
                .parseFromString(html, "text/html");

            const heading = documentPage.querySelector("h1");
            const content = documentPage.querySelector(".content");

            const titleText = heading
                ? heading.textContent.trim()
                : result.ref;

            const excerptText = content
                ? createExcerpt(content.textContent, query)
                : "";

            highlight(title, titleText, query);

            if (excerptText) {
                highlight(excerpt, excerptText, query);
            }

        } catch {
            title.textContent = result.ref;
        }

        if (generation !== getCurrentGeneration()) {
            return;
        }

        link.appendChild(title);

        if (excerpt.textContent) {
            link.appendChild(excerpt);
        }

        container.appendChild(link);
    }

    container.hidden = false;
}


function createExcerpt(text, query) {
    const cleanText = text
        .replace(/\s+/g, " ")
        .trim();

    const lowerText = cleanText.toLowerCase();
    const lowerQuery = query.toLowerCase();

    const index = lowerText.indexOf(lowerQuery);

    if (index === -1) {
        return cleanText.slice(0, 140) + "...";
    }

    const start = Math.max(0, index - 60);
    const end = Math.min(
        cleanText.length,
        index + query.length + 100
    );

    let excerpt = cleanText.slice(start, end);

    if (start > 0) {
        excerpt = "..." + excerpt;
    }

    if (end < cleanText.length) {
        excerpt += "...";
    }

    return excerpt;
}


function updateActiveResult(items, activeIndex) {
    items.forEach((item, index) => {
        item.classList.toggle(
            "active",
            index === activeIndex
        );
    });
}


function closeResults(container) {
    container.innerHTML = "";
    container.hidden = true;
}


function normalizeUrl(url) {
    try {
        return new URL(url).pathname;
    } catch {
        return url;
    }
}


function highlight(container, text, query) {
    const escapedQuery = query.replace(
        /[.*+?^${}()|[\]\\]/g,
        "\\$&"
    );

    const regex = new RegExp(`(${escapedQuery})`, "gi");
    const parts = text.split(regex);

    container.textContent = "";

    parts.forEach((part, index) => {
        if (index % 2 === 1) {
            const mark = document.createElement("mark");
            mark.textContent = part;
            container.appendChild(mark);
        } else {
            container.appendChild(
                document.createTextNode(part)
            );
        }
    });
}


function setupPageSearch(index, form, input, results) {
    form.addEventListener("submit", (event) => {
        event.preventDefault();

        const query = input.value.trim();

        if (!query) {
            results.innerHTML = "";
            return;
        }

        const searchResults = search(index, query);

        displayPageResults(
            searchResults,
            query,
            results
        );
    });
}


function displayPageResults(searchResults, query, container) {
    container.innerHTML = "";

    if (searchResults.length === 0) {
        container.innerHTML = "<p>No results found.</p>";
        return;
    }

    const list = document.createElement("ul");

    for (const result of searchResults) {
        const item = document.createElement("li");
        const link = document.createElement("a");

        link.href = normalizeUrl(result.ref);

        highlight(link, result.ref, query);

        item.appendChild(link);
        list.appendChild(item);
    }

    container.appendChild(list);
}