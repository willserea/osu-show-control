// -----------------------------
// DOM-Elemente holen
// -----------------------------

const statusEl = document.getElementById("status");
const logEl = document.getElementById("log");

const navButtons = document.querySelectorAll(".nav-button");
const pages = document.querySelectorAll(".page");

const osuEventSelect = document.getElementById("osu-event-select");
const eventSetupForm = document.getElementById("event-setup-form");
const oscAddressInput = document.getElementById("osc-address-input");
const oscTextInput = document.getElementById("osc-text-input");
const mappingList = document.getElementById("mapping-list");

const mappings = [];

// -----------------------------
// Sidebar / Seitenwechsel
// -----------------------------

navButtons.forEach((button) => {
  button.addEventListener("click", () => {
    const targetPage = button.dataset.page;
    const targetElement = document.getElementById(targetPage);

    if (!targetElement) {
      console.error("Seite nicht gefunden:", targetPage);
      return;
    }

    pages.forEach((page) => {
      page.classList.remove("active");
    });

    navButtons.forEach((navButton) => {
      navButton.classList.remove("active");
    });

    targetElement.classList.add("active");
    button.classList.add("active");
  });
});

// -----------------------------
// Event-Dropdown befüllen
// -----------------------------

function renderEventOptions(events) {
  if (!osuEventSelect) {
    console.warn("osu-event-select nicht gefunden");
    return;
  }

  osuEventSelect.innerHTML = '<option value="">Bitte Event auswählen</option>';

  events.forEach((eventDefinition) => {
    const option = document.createElement("option");

    option.value = eventDefinition.id;
    option.textContent = `${eventDefinition.label} (${eventDefinition.osc_address})`;

    option.dataset.jsonPointer = eventDefinition.json_pointer;
    option.dataset.oscAddress = eventDefinition.osc_address;
    option.dataset.valueType = eventDefinition.value_type;

    osuEventSelect.appendChild(option);
  });
}

// Wenn ein osu!-Event gewählt wird,
// automatisch die passende OSC-Adresse einsetzen.
if (osuEventSelect && oscAddressInput) {
  osuEventSelect.addEventListener("change", () => {
    const selectedOption = osuEventSelect.options[osuEventSelect.selectedIndex];

    if (!selectedOption) {
      return;
    }

    const oscAddress = selectedOption.dataset.oscAddress;

    if (oscAddress) {
      oscAddressInput.value = oscAddress;
    }
  });
}

// -----------------------------
// Mapping-Formular
// -----------------------------

if (eventSetupForm) {
  eventSetupForm.addEventListener("submit", (event) => {
    event.preventDefault();

    const selectedOption = osuEventSelect.options[osuEventSelect.selectedIndex];

    const osuEvent = osuEventSelect.value;
    const eventLabel = selectedOption ? selectedOption.textContent : osuEvent;

    const oscAddress = oscAddressInput.value.trim();
    const oscText = oscTextInput.value.trim();

    if (!osuEvent || !oscAddress) {
      alert("Bitte Event und OSC-Adresse auswählen.");
      return;
    }

    const mapping = {
      osuEvent,
      eventLabel,
      oscAddress,
      oscText,
    };

    mappings.push(mapping);
    renderMappings();

    eventSetupForm.reset();
  });
}

function renderMappings() {
  if (!mappingList) {
    return;
  }

  mappingList.innerHTML = "";

  mappings.forEach((mapping) => {
    const entry = document.createElement("div");
    entry.className = "mapping-entry";

    entry.innerHTML = `
                <strong>${mapping.eventLabel}</strong><br>
                OSC: ${mapping.oscAddress}<br>
                Text: ${mapping.oscText || "-"}
            `;

    mappingList.appendChild(entry);
  });
}

// -----------------------------
// Konsolen-Log
// -----------------------------

function addLogEntry(address, value) {
  if (!logEl) {
    return;
  }

  const now = new Date().toLocaleTimeString();

  const entry = document.createElement("div");
  entry.className = "entry";

  entry.innerHTML = `
            <span class="time">${now}</span>
            <span class="address">${address}</span>
            <span class="value">${value}</span>
        `;

  logEl.prepend(entry);
}

// -----------------------------
// Statusanzeige
// -----------------------------

function setStatus(text, className) {
  if (!statusEl) {
    return;
  }

  statusEl.textContent = text;
  statusEl.className = className;
}

// -----------------------------
// WebSocket-Verbindung zu Rust
// -----------------------------

const socket = new WebSocket("ws://127.0.0.1:3001");

socket.addEventListener("open", () => {
  setStatus("Verbunden", "connected");
  addLogEntry("/system/websocket", "Verbunden");
});

socket.addEventListener("close", () => {
  setStatus("Verbindung geschlossen", "disconnected");
  addLogEntry("/system/websocket", "Verbindung geschlossen");
});

socket.addEventListener("error", () => {
  setStatus("WebSocket Fehler", "error");
  addLogEntry("/system/websocket", "Fehler");
});

socket.addEventListener("message", (event) => {
  let address = "event";
  let value = event.data;

  try {
    const data = JSON.parse(event.data);

    // Spezialnachricht vom Rust-Backend:
    // Liste aller verfügbaren osu!-Events.
    if (data.type === "event_definitions") {
      renderEventOptions(data.events);
      addLogEntry("/system/events", `${data.events.length} Events geladen`);
      return;
    }

    // Normale Live-Events, z. B.:
    // {"address":"/osu/mode","value":"mania"}
    address = data.address ?? "event";
    value = data.value ?? "";
  } catch (_) {
    // Falls kein JSON kommt, bleibt event.data als normaler Text erhalten.
  }

  addLogEntry(address, value);
});
