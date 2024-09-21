const { invoke } = window.__TAURI__.tauri;

const Colors = {
  StartCell: "green",
  EndCell: "red",
  EmptyCell: "white",
  BlockCell: "black",
  PathCell: "blue",

  ButtonBackgroundInactive: "white",
  ButtonBackgroundActive: "lightblue",
};

const ApplicationState = {
  Ready: "Ready",
  Drawing: "Drawing",
  Executed: "Executed",
};

const CellEditorMode = {
  Empty: "Empty",
  Block: "Block",
  Start: "Start",
  End: "End",
};

let appState = ApplicationState.Ready;
let cellEditMode = CellEditorMode.Empty;

let isMouseDown = false;

let editorModeSelect;
let algorithmSelect;

let runButton;

let gridSizeInput;
let cellSizeInput;
let createGridButton;
let clearGridButton;
let gridContainer;
let grid;

let startCell;
let endCell;

window.addEventListener("DOMContentLoaded", () => {
  loadElements();
  addEventListeners();
  initializeElements();
});

window.addEventListener("mousedown", () => {
  if (appState === ApplicationState.Executed) {
    clearPathCells();
    appState = ApplicationState.Ready;
  }
});

function loadElements() {
  editorModeSelect = document.querySelector("#editor-mode");
  algorithmSelect = document.querySelector("#algorithm-select");

  runButton = document.querySelector("#run-button");

  gridSizeInput = document.querySelector("#grid-size-input");
  cellSizeInput = document.querySelector("#cell-size-input");
  createGridButton = document.querySelector("#create-grid-button");
  clearGridButton = document.querySelector("#clear-grid-button");
  gridContainer = document.getElementById("grid-container");
  grid = document.getElementById("grid");
}

function addEventListeners() {
  addControlEventListeners();
  addGridEventListeners();
}

function addControlEventListeners() {
  const modeStrToEnum = (mode) => {
    switch (mode) {
      case CellEditorMode[CellEditorMode.Empty]:
        return CellEditorMode.Empty;
      case CellEditorMode[CellEditorMode.Block]:
        return CellEditorMode.Block;
      case CellEditorMode[CellEditorMode.Start]:
        return CellEditorMode.Start;
      case CellEditorMode[CellEditorMode.End]:
        return CellEditorMode.End;
    }
  };

  editorModeSelect.addEventListener("change", () => {
    cellEditMode = modeStrToEnum(editorModeSelect.value);
  });

  runButton.addEventListener("click", () => {
    if (appState != ApplicationState.Ready) {
      return;
    }

    if (!startCell || !endCell) {
      alert("Please select a START and END cell");
      return;
    }

    const colors = [];
    const costs = [];
    for (let i = 0; i < grid.children.length; i++) {
      colors.push(grid.children[i].style.backgroundColor);
      costs.push(parseInt(grid.children[i].textContent));
    }

    const algorithm = algorithmSelect.value;
    const gridSize = gridSizeInput.value;

    appState = ApplicationState.Drawing;
    findAndDrawShortestPath(colors, costs, gridSize, algorithm).then(() => {
      appState = ApplicationState.Executed;
    });
  });
}

async function findAndDrawShortestPath(colors, costs, gridSize, algorithm) {
  const path = await invoke("run_pathfinding", {
    colors: colors,
    costs: costs,
    gridSize: gridSize,
    algorithm: algorithm,
  });

  if (path.length === 0) {
    alert("END is not reachable from START");
  }

  for (const step of path.slice(1, path.length - 1)) {
    // row * gridSize + col
    const idx = step[0] * gridSize + step[1];

    grid.children[idx].style.backgroundColor = Colors.PathCell;
  }
}

function addGridEventListeners() {
  createGridButton.addEventListener("click", createGrid);
  clearGridButton.addEventListener("click", () => {
    clearGrid();
    startCell = null;
    endCell = null;
  });
}

function createGrid() {
  startCell = null;
  endCell = null;

  while (grid.firstChild) {
    grid.firstChild.remove();
  }

  const gridSize = gridSizeInput.value;
  const cellSize = cellSizeInput.value;

  grid.style.width = `${gridSize * cellSize}px`;
  grid.style.height = `${gridSize * cellSize}px`;
  grid.style.gridTemplateColumns = `repeat(${gridSize}, 1fr)`;
  grid.style.gridTemplateRows = `repeat(${gridSize}, 1fr)`;

  for (let i = 0; i < gridSize; i++) {
    for (let j = 0; j < gridSize; j++) {
      const gridItem = document.createElement("div");
      const cost = Math.floor(Math.random() * 10) + 1;
      gridItem.textContent = cost;
      gridItem.classList.add("grid-item");
      gridItem.style.backgroundColor = Colors.EmptyCell;

      const events = ["mousedown", "mousemove"];

      events.forEach((event) => {
        gridItem.addEventListener(event, () => {
          if (event === "mousemove" && !isMouseDown) return;

          if (
            startCell &&
            gridItem === startCell &&
            cellEditMode != CellEditorMode.Start
          ) {
            startCell.style.backgroundColor = Colors.EmptyCell;
            startCell = null;
          }
          if (
            endCell &&
            gridItem === endCell &&
            cellEditMode != CellEditorMode.End
          ) {
            endCell.style.backgroundColor = Colors.EmptyCell;
            endCell = null;
          }

          switch (cellEditMode) {
            case CellEditorMode.Empty:
              gridItem.style.backgroundColor = Colors.EmptyCell;
              break;
            case CellEditorMode.Block:
              gridItem.style.backgroundColor = Colors.BlockCell;
              break;
            case CellEditorMode.Start:
              if (startCell && gridItem != startCell) {
                startCell.style.backgroundColor = Colors.EmptyCell;
              }
              startCell = gridItem;
              gridItem.style.backgroundColor = Colors.StartCell;
              break;
            case CellEditorMode.End:
              if (endCell && gridItem != endCell) {
                endCell.style.backgroundColor = Colors.EmptyCell;
              }
              endCell = gridItem;
              gridItem.style.backgroundColor = Colors.EndCell;
              break;
          }
        });
      });

      grid.appendChild(gridItem);
    }
  }
}

function initializeElements() {
  cellEditMode = editorModeSelect.value;

  // Create default grid
  createGridButton.click();

  gridContainer.addEventListener("mousedown", () => {
    isMouseDown = true;
  });
  gridContainer.addEventListener("mouseup", () => {
    isMouseDown = false;
  });
}

function clearGrid() {
  for (const cell of grid.children) {
    cell.style.backgroundColor = Colors.EmptyCell;
  }
}

function clearPathCells() {
  for (const cell of grid.children) {
    if (cell.style.backgroundColor === Colors.PathCell) {
      cell.style.backgroundColor = Colors.EmptyCell;
    }
  }
}
