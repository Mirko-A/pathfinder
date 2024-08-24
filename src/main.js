const { invoke } = window.__TAURI__.tauri;

const SquareColor = {
  Empty: "white",
  Block: "black",
  Start: "green",
  End: "red",
};

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

let runButton;
let debugButton;

let gridSizeInput;
let squareSizeInput;
let createGridButton;
let gridContainer;
let grid;

let startSquare;
let endSquare;

function loadElements() {
  emptyModeButton = document.querySelector("#empty-mode-button");
  blockModeButton = document.querySelector("#block-mode-button");
  startModeButton = document.querySelector("#start-mode-button");
  endModeButton = document.querySelector("#end-mode-button");

  runButton = document.querySelector("#run-button");
  debugButton = document.querySelector("#debug-button");

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

  runButton.addEventListener("click", () => {});
  debugButton.addEventListener("click", () => {
    if (!startSquare || !endSquare) {
      alert("Please select a START and END square");
      return;
    }

    const colors = [];
    for (let i = 0; i < grid.children.length; i++) {
      colors.push(grid.children[i].style.backgroundColor);
    }

    invoke("debug", { colors: colors, gridSize: gridSizeInput.value }).then(
      (path) => {
        for (const step of path) {
          // step was tuple in rust (usize, usize), convert to index in grid
          const idx = step[0] * gridSizeInput.value + step[1];
          grid.children[idx].style.backgroundColor = "blue";
          // wait for 100ms
          new Promise((resolve) => setTimeout(resolve, 100));
        }
      },
    );
  });
}

function addGridEventListeners() {
  createGridButton.addEventListener("click", () => {
    startSquare = null;
    endSquare = null;

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
        gridItem.style.backgroundColor = SquareColor.Empty;

        const events = ["mousedown", "mousemove"];

        events.forEach((event) => {
          gridItem.addEventListener(event, () => {
            if (event == "mousemove" && !isMouseDown) return;

            if (
              startSquare &&
              gridItem == startSquare &&
              mode != GridEditorMode.Start
            ) {
              startSquare.style.backgroundColor = SquareColor.Empty;
              startSquare = null;
            }
            if (
              endSquare &&
              gridItem == endSquare &&
              mode != GridEditorMode.End
            ) {
              endSquare.style.backgroundColor = SquareColor.Empty;
              endSquare = null;
            }

            switch (mode) {
              case GridEditorMode.Empty:
                gridItem.style.backgroundColor = SquareColor.Empty;
                break;
              case GridEditorMode.Block:
                gridItem.style.backgroundColor = SquareColor.Block;
                break;
              case GridEditorMode.Start:
                if (startSquare && gridItem != startSquare) {
                  startSquare.style.backgroundColor = SquareColor.Empty;
                }
                startSquare = gridItem;
                gridItem.style.backgroundColor = SquareColor.Start;
                break;
              case GridEditorMode.End:
                if (endSquare && gridItem != endSquare) {
                  endSquare.style.backgroundColor = SquareColor.Empty;
                }
                endSquare = gridItem;
                gridItem.style.backgroundColor = SquareColor.End;
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
