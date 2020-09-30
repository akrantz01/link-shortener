const basePath = "/ui/api";

window.onload = async () => {
    // Fetch initial set of links
    await fetchLinks();

    // Register event listeners
    document.getElementById("navbar-create").addEventListener("click", () => halfmoon.toggleModal("create-modal"));
    document.getElementById("navbar-refresh").addEventListener("click", fetchLinks);
    document.getElementById("navbar-create-small").addEventListener("click", () => halfmoon.toggleModal("create-modal"));
    document.getElementById("navbar-refresh-small").addEventListener("click", fetchLinks);
    document.getElementById("create-modal-form").addEventListener("submit", createLink);
    document.getElementById("create-modal-cancel").addEventListener("click", () => halfmoon.toggleModal("create-modal"));
    document.getElementById("edit-modal-form").addEventListener("submit", editLink);
    document.getElementById("edit-modal-cancel").addEventListener("click", () => halfmoon.toggleModal("edit-modal"));
    document.getElementById("delete-confirm-button").addEventListener("click", deleteLink);
    document.getElementById("delete-confirm-cancel").addEventListener("click", () => halfmoon.toggleModal("delete-confirm-modal"));
};

// Retrieve a list of all the links in the database
async function fetchLinks() {
    // Enable the spinner
    const spinner = document.getElementById("links-loading");
    spinner.style.display = "block";

    let links;
    try {
        // Send the API call
        const response = await fetch(basePath);
        links = await response.json();

        // Ensure successful
        if (!links.success) {
            halfmoon.initStickyAlert({
                content: links.message,
                title: "Failed to load links",
                alertType: "alert-danger"
            });
            return;
        }
    } catch (e) {
        // Notify error
        halfmoon.initStickyAlert({
            content: e.message,
            title: "Failed to load links",
            alertType: "alert-danger"
        });
        return;
    }

    // Clear table rows
    const table = document.getElementById("links-content");
    while (table.lastElementChild) table.removeChild(table.lastElementChild);

    // Generate table rows
    links.data.forEach(link => table.appendChild(createRow(link)));

    // Disable the spinner
    spinner.style.display = "none";
}

// API request for creating a new link
async function createLink(e) {
    e.preventDefault();

    // Close the modal
    halfmoon.toggleModal("create-modal");

    // Retrieve the values from the form
    const name = getAndClearField("create-link-name").toLocaleLowerCase();
    const link = getAndClearField("create-link-url");

    let createdLink;
    try {
        // Send API call
        const response = await fetch(basePath, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "name": name,
                "link": link
            })
        });
        createdLink = await response.json();

        // Ensure successful
        if (!createdLink.success) {
            halfmoon.initStickyAlert({
                content: createdLink.message,
                title: "Failed to load links",
                alertType: "alert-danger"
            });
            return;
        }
    } catch (e) {
        // Notify error
        halfmoon.initStickyAlert({
            content: e.message,
            title: "Failed to add link",
            alertType: "alert-danger"
        });
        return;
    }

    // Append the link to the table
    document.getElementById("links-content").appendChild(createRow(createdLink.data));
}

// API request for editing a link
async function editLink(e) {
    e.preventDefault();

    // Close the modal
    halfmoon.toggleModal("edit-modal");

    // Get the name of the link
    const id = document.getElementById("edit-link-id").innerText;
    const numericId = id.split("_")[1];

    // Retrieve the values from the form
    const name = getAndClearField("edit-link-name").toLocaleLowerCase();
    const link = getAndClearField("edit-link-url");
    const enabled = document.getElementById("edit-link-enabled").checked;

    // Build the request body
    const body = { enabled };
    if (name !== "") body.name = name;
    if (link !== "") body.link = link;

    try {
        // Send API call
        const response = await fetch(`${basePath}/${numericId}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(body)
        });

        // Ensure successful
        if (response.status !== 204) {
            // Parse the response body
            const json = await response.json();

            // Display a toast
            halfmoon.initStickyAlert({
                content: json.message,
                title: "Unable to update link",
                alertType: "alert-danger"
            });
            return;
        }
    } catch (e) {
        halfmoon.initStickyAlert({
            content: e.message,
            title: "Unable to update link",
            alertType: "alert-danger"
        });
        return;
    }

    // Update the table values
    const table = document.getElementById("links-content");
    for (const row of table.children) if (row.id === id) {
        // Change the name
        if (name !== "") document.getElementById(`${id}_name`).innerText = name;

        // Change the link
        if (link !== "") {
            const linkElement = document.getElementById(`${id}_link`);
            linkElement.href = link;
            linkElement.innerText = link;
        }

        // Change enabled status
        const statusElement = document.getElementById(`${id}_enabled`);
        statusElement.classList.remove("fa-check", "fa-times");
        statusElement.classList.add(`fa-${enabled ? "check" : "times"}`);
    }
}

// API request for deleting a link
async function deleteLink() {
    // Close the modal
    halfmoon.toggleModal("delete-confirm-modal");

    // Get the name of the link
    const id = document.getElementById("delete-link-id").innerText;
    const numericId = id.split("_")[1];

    try {
        // Send API call
        const response = await fetch(`${basePath}/${numericId}`, { method: "DELETE" });

        // Ensure successful
        if (response.status !== 204) {
            // Parse the response body
            const json = await response.json();

            // Display a toast
            halfmoon.initStickyAlert({
                content: json.message,
                title: "Unable to delete link",
                alertType: "alert-danger"
            });
            return;
        }
    } catch (e) {
        halfmoon.initStickyAlert({
            content: e.message,
            title: "Unable to delete link",
            alertType: "alert-danger"
        });
        return;
    }

    // Remove the element from the table
    const table = document.getElementById("links-content");
    for (const row of table.children) if (row.id === id) table.removeChild(row);
}

// Create a table row from a link's data
function createRow(link) {
    const baseId = `link_${link.id}`;

    // Name cell
    const name = document.createElement("td");
    name.id = baseId + "_name"
    name.appendChild(document.createTextNode(link.name));

    // URL cell (cell + anchor tag)
    const urlAnchor = document.createElement("a");
    urlAnchor.id = baseId + "_link";
    urlAnchor.href = link.link;
    urlAnchor.target = "_blank";
    urlAnchor.appendChild(document.createTextNode(link.link));

    const url = document.createElement("td");
    url.appendChild(urlAnchor);

    // Enabled/disabled cell (cell + icon)
    const enabled = document.createElement("td");
    enabled.appendChild(fontAwesomeIcon(link.enabled ? "check" : "times", baseId + "_enabled"));

    // Times used cell
    const timesUsed = document.createElement("td");
    timesUsed.appendChild(document.createTextNode(link.times_used));

    // Actions cell (cell + button group + edit and delete buttons)
    const editButton = document.createElement("button");
    editButton.id = `${baseId}_buttons_edit`;
    editButton.classList.add("btn", "btn-square", "btn-md", "btn-primary");
    editButton.type = "button";
    editButton.dataset.toggle = "tooltip";
    editButton.dataset.title = "Edit";
    editButton.appendChild(fontAwesomeIcon("edit"));
    editButton.addEventListener("click", editClickHandler(baseId));

    const deleteButton = document.createElement("button");
    deleteButton.id = `${baseId}_buttons_delete`;
    deleteButton.classList.add("btn", "btn-square", "btn-md", "btn-danger");
    deleteButton.type = "button";
    deleteButton.dataset.toggle = "tooltip";
    deleteButton.dataset.title = "Delete";
    deleteButton.appendChild(fontAwesomeIcon("trash"));
    deleteButton.addEventListener("click", deleteClickHandler(baseId));

    const buttonGroup = document.createElement("div");
    buttonGroup.classList.add("btn-group");
    buttonGroup.appendChild(editButton);
    buttonGroup.appendChild(deleteButton);

    const actions = document.createElement("td");
    actions.classList.add("text-right");
    actions.appendChild(buttonGroup);

    // Containing table row
    const row = document.createElement("tr");
    row.id = baseId;
    row.appendChild(name);
    row.appendChild(url);
    row.appendChild(enabled);
    row.appendChild(timesUsed);
    row.appendChild(actions);

    return row;
}

// Handler for the edit button being pressed
const editClickHandler = id => () => {
    // Retrieve fields to be filled in
    const name = document.getElementById(`${id}_name`).innerText;
    const link = document.getElementById(`${id}_link`).innerText;
    const enabled = document.getElementById(`${id}_enabled`).classList.contains("fa-check");

    // Fill in field values
    document.getElementById("edit-link-display-name").innerText = name;
    document.getElementById("edit-link-id").innerText = id;
    document.getElementById("edit-link-name").placeholder = name;
    document.getElementById("edit-link-url").placeholder = link;
    document.getElementById("edit-link-enabled").checked = enabled;

    // Open the modal
    halfmoon.toggleModal("edit-modal");
};

// Handler for the delete button being pressed
const deleteClickHandler = id => () => {
    // Fill in link name and id
    document.getElementById("delete-link-id").innerText = id;
    document.getElementById("delete-link-name").innerText =
        document.getElementById(`${id}_name`).innerText;

    // Open the modal
    halfmoon.toggleModal("delete-confirm-modal");
};

// Generate an element for a FontAwesome icon
function fontAwesomeIcon(name, id="") {
    const icon = document.createElement("i");
    if (id !== "") icon.id = id;
    icon.classList.add("fas", `fa-${name}`);
    return icon;
}

// Retrieve the value of a field and reset it
function getAndClearField(id) {
    const field = document.getElementById(id);
    setTimeout(() => field.value = "", 10);
    return field.value;
}
