const { invoke } = window.__TAURI__.tauri;

const GridEditorMode = {
  Empty: "Empty",
  Block: "Block",
  Start: "Start",
  End: "End",
};

let mode = GridEditorMode.Empty;
let isMouseDown = false;

let emptyModeButton;
let blockModeButton;
let startModeButton;
let endModeButton;

let gridSizeInput;
let squareSizeInput;
let createGridButton;
let gridContainer;
let grid;

// TODO: Implement the functionality to set start and end squares
let startSquare = null;
let endSquare = null;

function loadElements() {
  emptyModeButton = document.querySelector("#empty-mode-button");
  blockModeButton = document.querySelector("#block-mode-button");
  startModeButton = document.querySelector("#start-mode-button");
  endModeButton = document.querySelector("#end-mode-button");

  gridSizeInput = document.querySelector("#grid-size-input");
  squareSizeInput = document.querySelector("#square-size-input");
  createGridButton = document.querySelector("#create-grid-button");
  gridContainer = document.getElementById("grid-container");
  grid = document.getElementById("grid");
}

function addEventListeners() {
  addSidebarEventListeners();
  addGridEventListeners();
}

function addSidebarEventListeners() {
  const modeToButton = (mode) => {
    switch (mode) {
      case GridEditorMode.Empty:
        return emptyModeButton;
      case GridEditorMode.Block:
        return blockModeButton;
      case GridEditorMode.Start:
        return startModeButton;
      case GridEditorMode.End:
        return endModeButton;
    }
  };

  emptyModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = "white";
    mode = GridEditorMode.Empty;
    emptyModeButton.style.backgroundColor = "lightblue";
  });
  blockModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = "white";
    mode = GridEditorMode.Block;
    blockModeButton.style.backgroundColor = "lightblue";
  });
  startModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = "white";
    mode = GridEditorMode.Start;
    startModeButton.style.backgroundColor = "lightblue";
  });
  endModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = "white";
    mode = GridEditorMode.End;
    endModeButton.style.backgroundColor = "lightblue";
  });
}

function addGridEventListeners() {
  createGridButton.addEventListener("click", () => {
    while (grid.firstChild) {
      grid.firstChild.remove();
    }

    const gridSize = gridSizeInput.value;
    const squareSize = squareSizeInput.value;

    grid.style.width = `${gridSize * squareSize}px`;
    grid.style.height = `${gridSize * squareSize}px`;
    grid.style.gridTemplateColumns = `repeat(${gridSize}, 1fr)`;
    grid.style.gridTemplateRows = `repeat(${gridSize}, 1fr)`;

    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        const gridItem = document.createElement("div");
        gridItem.classList.add("grid-item");

        const events = ["mousedown", "mousemove"];
        events.forEach((event) => {
          gridItem.addEventListener(event, () => {
            if (event == "mousemove" && !isMouseDown) return;

            switch (mode) {
              case GridEditorMode.Empty:
                gridItem.style.backgroundColor = "white";
                break;
              case GridEditorMode.Block:
                gridItem.style.backgroundColor = "black";
                break;
              case GridEditorMode.Start:
                gridItem.style.backgroundColor = "green";
                break;
              case GridEditorMode.End:
                gridItem.style.backgroundColor = "red";
                break;
            }
          });
        });

        grid.appendChild(gridItem);
      }
    }
  });
}

function initializeElements() {
  // Assuming default mode is Empty
  emptyModeButton.style.backgroundColor = "lightblue";
  // Create default grid
  createGridButton.click();

  gridContainer.addEventListener("mousedown", () => {
    isMouseDown = true;
  });
  gridContainer.addEventListener("mouseup", () => {
    isMouseDown = false;
  });
}

window.addEventListener("DOMContentLoaded", () => {
  loadElements();
  addEventListeners();
  initializeElements();
});
