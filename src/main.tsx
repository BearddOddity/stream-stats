import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./index.css";

// Standalone default theme — when vendored into StreamerSuite, these are
// overridden by the shared SharedSettingsContext instead.
document.documentElement.style.setProperty("--accent-system", "#9146ff");

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
