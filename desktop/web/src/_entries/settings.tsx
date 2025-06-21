import React from "react";
import ReactDOM from "react-dom/client";
import "../_utils/import-daisyui.css";

function App() {
  // tabs: tray, widget, general
  // tray: text(net-speed, fixed).
  // widget: visible(toggle), position(tr,tl,br,bl) + safe-area(toggle), size(slider), refresh speed(range with steps).
  // general: auto-start(toggle), send usage telemetry(toggle), star on github

  return (
    <div className="flex-1 tabs tabs-lg tabs-border">
      <input
        type="radio"
        name="tab"
        className="tab"
        aria-label="General"
        defaultChecked
      />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        General content
      </div>

      <input type="radio" name="tab" className="tab" aria-label="Widget" />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        Widget content
      </div>

      <input type="radio" name="tab" className="tab" aria-label="Tray" />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        Tray content
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
