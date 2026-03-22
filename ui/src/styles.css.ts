import { globalStyle } from "@vanilla-extract/css";

globalStyle(":root", {
  colorScheme: "dark",
  fontFamily: '"IBM Plex Sans", "Segoe UI", sans-serif',
  lineHeight: "1.5",
  fontWeight: "400",
  color: "#f2efe7",
  fontSynthesis: "none",
  textRendering: "optimizeLegibility",
  WebkitFontSmoothing: "antialiased",
  MozOsxFontSmoothing: "grayscale",
  vars: {
    "--border-color": "rgba(255, 255, 255, 0.12)",
    "--panel-background": "rgba(15, 23, 42, 0.78)",
    "--panel-shadow": "0 24px 80px rgba(15, 23, 42, 0.4)",
    "--accent": "#8bf5d4",
    "--accent-strong": "#4fd6ac",
    "--danger": "#ffb4ab",
    "--muted": "rgba(242, 239, 231, 0.72)",
  },
});

globalStyle("*", {
  boxSizing: "border-box",
});

globalStyle("html, body", {
  margin: 0,
  width: "100%",
  height: "100%",
  minWidth: "320px",
  minHeight: "100%",
  overflow: "hidden",
  background:
    "radial-gradient(circle at top, rgba(77, 132, 255, 0.2), transparent 32%), linear-gradient(180deg, #111827 0%, #172033 55%, #0f172a 100%)",
});

globalStyle("a", {
  color: "inherit",
});

globalStyle("button, input, textarea", {
  font: "inherit",
});

globalStyle("button", {
  cursor: "pointer",
});

globalStyle("code, pre", {
  fontFamily: '"IBM Plex Mono", "SFMono-Regular", monospace',
});

globalStyle("#root", {
  width: "100%",
  height: "100%",
  minHeight: "100%",
  overflow: "hidden",
  background: "transparent",
});

globalStyle(".button-link, .nosqo-button", {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "flex-start",
  gap: "0.4rem",
  border: "1px solid var(--border-color)",
  borderRadius: "999px",
  padding: "0.6rem 0.9rem",
  background: "rgba(255, 255, 255, 0.04)",
  color: "inherit",
  textDecoration: "none",
  transition: "transform 160ms ease, border-color 160ms ease, background 160ms ease",
});

globalStyle(".button-link:hover, .nosqo-button:hover", {
  transform: "translateY(-1px)",
  borderColor: "rgba(139, 245, 212, 0.5)",
});

globalStyle(".nosqo-button", {
  borderColor: "rgba(139, 245, 212, 0.28)",
  background: "rgba(139, 245, 212, 0.12)",
  color: "#f8fafc",
});

globalStyle(".nosqo-button:disabled", {
  cursor: "not-allowed",
  opacity: "0.65",
});

globalStyle(".nosqo-panel", {
  border: "1px solid var(--border-color)",
  borderRadius: "20px",
  padding: "1rem",
  background: "var(--panel-background)",
  boxShadow: "var(--panel-shadow)",
  backdropFilter: "blur(24px)",
});

globalStyle(".panel-stack", {
  display: "grid",
  gap: "0.75rem",
});

globalStyle(".page-title", {
  margin: 0,
  fontSize: "clamp(2rem, 4vw, 3.25rem)",
  lineHeight: "1",
});

globalStyle(".body-copy", {
  margin: 0,
  color: "var(--muted)",
});

globalStyle(".kicker", {
  margin: 0,
  textTransform: "uppercase",
  letterSpacing: "0.18em",
  fontSize: "0.8rem",
  color: "var(--accent)",
});

globalStyle(".hint", {
  margin: 0,
  color: "var(--muted)",
});

globalStyle(".nosqo-shell", {
  display: "grid",
  gridTemplateRows: "44px minmax(0, 1fr)",
  width: "100%",
  height: "100vh",
  minHeight: "100vh",
  overflow: "hidden",
  background: "transparent",
});

globalStyle(".shell-header, .shell-navbar", {
  borderColor: "var(--border-color)",
  background: "rgba(8, 15, 28, 0.88)",
  backdropFilter: "blur(24px)",
});

globalStyle(".shell-header__row", {
  display: "flex",
  height: "100%",
  padding: "0 0.85rem",
  alignItems: "center",
  justifyContent: "flex-start",
  flexWrap: "nowrap",
  gap: "0.75rem",
});

globalStyle(".shell-burger", {
  display: "none",
  flex: "0 0 auto",
  width: "2rem",
  height: "2rem",
  padding: 0,
  border: 0,
  borderRadius: "0.5rem",
  background: "transparent",
});

globalStyle(".shell-burger__lines, .shell-burger__lines::before, .shell-burger__lines::after", {
  display: "block",
  width: "1rem",
  height: "2px",
  background: "#f8fafc",
  transition: "transform 160ms ease, opacity 160ms ease",
  content: '""',
});

globalStyle(".shell-burger__lines", {
  position: "relative",
  margin: "0 auto",
});

globalStyle(".shell-burger__lines::before", {
  position: "absolute",
  top: "-0.35rem",
});

globalStyle(".shell-burger__lines::after", {
  position: "absolute",
  top: "0.35rem",
});

globalStyle(".shell-burger__lines--open", {
  background: "transparent",
});

globalStyle(".shell-burger__lines--open::before", {
  transform: "translateY(0.35rem) rotate(45deg)",
});

globalStyle(".shell-burger__lines--open::after", {
  transform: "translateY(-0.35rem) rotate(-45deg)",
});

globalStyle(".brand-copy", {
  display: "flex",
  gap: "0.35rem",
  alignItems: "baseline",
  flex: "0 0 auto",
});

globalStyle(".brand-link", {
  color: "#f8fafc",
  fontSize: "1rem",
  fontWeight: "700",
  textDecoration: "none",
});

globalStyle(".breadcrumb-list", {
  display: "flex",
  gap: "0.45rem",
  listStyle: "none",
  margin: 0,
  padding: 0,
  maxWidth: "min(52vw, 42rem)",
  whiteSpace: "nowrap",
  overflow: "hidden",
});

globalStyle(".breadcrumb-item", {
  display: "inline-flex",
  gap: "0.45rem",
  alignItems: "center",
  fontSize: "0.86rem",
  minWidth: 0,
});

globalStyle(".breadcrumb-item:not(:last-child)::after", {
  color: "var(--muted)",
  content: '"/"',
});

globalStyle(".breadcrumb-link", {
  color: "var(--muted)",
  textDecoration: "none",
});

globalStyle(".breadcrumb-link:hover", {
  color: "#f8fafc",
});

globalStyle(".breadcrumb-current", {
  color: "#f8fafc",
  fontWeight: "600",
});

globalStyle(".shell-body", {
  display: "grid",
  gridTemplateColumns: "320px minmax(0, 1fr)",
  height: "100%",
  minHeight: 0,
  overflow: "hidden",
  background: "transparent",
});

globalStyle(".shell-navbar__scroll", {
  height: "100%",
  overflowY: "auto",
  padding: "0.85rem",
});

globalStyle(".nav-group-label", {
  margin: "0 0 0.5rem",
  color: "var(--accent)",
  fontSize: "0.78rem",
  fontWeight: "700",
  letterSpacing: "0.16em",
  textTransform: "uppercase",
});

globalStyle(".nav-list", {
  display: "grid",
  gap: "0.5rem",
});

globalStyle(".nav-link", {
  display: "block",
  padding: "0.55rem 0.15rem 0.55rem 0.75rem",
  border: 0,
  borderLeft: "2px solid transparent",
  borderRadius: 0,
  background: "transparent",
  color: "#f8fafc",
  textDecoration: "none",
  transition: "transform 160ms ease, border-color 160ms ease, background 160ms ease",
});

globalStyle(".nav-link:hover", {
  transform: "translateY(-1px)",
  borderLeftColor: "rgba(139, 245, 212, 0.45)",
  background: "rgba(139, 245, 212, 0.05)",
});

globalStyle(".nav-link--active", {
  borderLeftColor: "rgba(139, 245, 212, 0.72)",
  background: "rgba(139, 245, 212, 0.1)",
});

globalStyle(".nav-link__label", {
  display: "block",
  color: "#f8fafc",
  fontSize: "0.92rem",
  fontWeight: "600",
  whiteSpace: "nowrap",
  overflow: "hidden",
  textOverflow: "ellipsis",
});

globalStyle(".shell-main", {
  display: "flex",
  flexDirection: "column",
  height: "100%",
  minHeight: 0,
  background: "transparent",
  overflow: "hidden",
  position: "relative",
});

globalStyle(".shell-main__viewport", {
  display: "flex",
  flex: "1 1 auto",
  flexDirection: "column",
  height: "100%",
  minHeight: 0,
  overflow: "hidden",
  background: "transparent",
});

globalStyle(".shell-main__content", {
  display: "grid",
  flex: "1 1 auto",
  gridTemplateRows: "minmax(0, 1fr)",
  height: "100%",
  width: "min(1200px, 100%)",
  margin: "0 auto",
  background: "transparent",
  padding: "0.75rem",
  minHeight: 0,
  overflow: "hidden",
});

globalStyle(".shell-main__content > *", {
  minHeight: 0,
});

globalStyle(".shell-backdrop", {
  display: "none",
});

globalStyle(".field", {
  display: "grid",
  gap: "0.35rem",
});

globalStyle(".field span", {
  fontSize: "0.95rem",
  color: "var(--muted)",
});

globalStyle(".field input, .field textarea", {
  width: "100%",
  border: "1px solid var(--border-color)",
  borderRadius: "16px",
  padding: "0.7rem 0.85rem",
  background: "rgba(15, 23, 42, 0.75)",
  color: "inherit",
});

globalStyle(".nosqo-input", {
  color: "#f2efe7",
  background: "rgba(15, 23, 42, 0.82)",
});

globalStyle(".nosqo-input::placeholder", {
  color: "rgba(242, 239, 231, 0.4)",
});

globalStyle(".nosqo-textarea", {
  minHeight: "9rem",
});

globalStyle(".field textarea", {
  resize: "vertical",
});

globalStyle(".toolbar", {
  display: "flex",
  flexWrap: "wrap",
  gap: "0.5rem",
  alignItems: "center",
});

globalStyle(".toolbar--filters", {
  alignItems: "end",
});

globalStyle(".nosqo-error-alert", {
  border: "1px solid rgba(255, 180, 171, 0.4)",
  borderRadius: "20px",
  padding: "1rem",
  background: "rgba(127, 29, 29, 0.18)",
});

globalStyle(".nosqo-badge", {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  padding: "0.22rem 0.55rem",
  border: "1px solid rgba(139, 245, 212, 0.28)",
  borderRadius: "999px",
  background: "rgba(139, 245, 212, 0.12)",
  color: "#b7f7e4",
  fontSize: "0.72rem",
  fontWeight: "700",
  letterSpacing: "0.06em",
  textTransform: "uppercase",
  whiteSpace: "nowrap",
});

globalStyle(".nosqo-badge--predicate", {
  borderColor: "rgba(103, 232, 249, 0.28)",
  background: "rgba(103, 232, 249, 0.12)",
  color: "#b7f6ff",
});

globalStyle(".empty-state, .table-shell, .code-block", {
  border: "1px solid var(--border-color)",
  borderRadius: "20px",
  background: "rgba(5, 24, 44, 0.72)",
});

globalStyle(".empty-state", {
  padding: "1rem",
});

globalStyle(".empty-state h3, .feature-card h3", {
  marginTop: 0,
});

globalStyle(".table-shell", {
  overflowX: "auto",
});

globalStyle(".code-block", {
  margin: 0,
  padding: "0.85rem",
  overflow: "auto",
  whiteSpace: "pre-wrap",
});

globalStyle("table", {
  width: "100%",
  borderCollapse: "collapse",
});

globalStyle("th, td", {
  padding: "0.65rem 0.8rem",
  textAlign: "left",
  borderBottom: "1px solid var(--border-color)",
});

globalStyle("th", {
  color: "var(--accent)",
  fontWeight: "600",
});

globalStyle("tbody tr:last-child td", {
  borderBottom: 0,
});

globalStyle(".filters-grid", {
  display: "grid",
  gap: "0.75rem",
});

globalStyle(".feature-grid", {
  display: "grid",
  gap: "0.85rem",
});

globalStyle(".feature-card", {
  border: "1px solid var(--border-color)",
  borderRadius: "20px",
  padding: "1rem",
  background: "rgba(255, 255, 255, 0.03)",
});

globalStyle(".feature-link", {
  display: "inline-block",
  marginTop: "0.5rem",
  color: "var(--accent)",
});

globalStyle(".sr-only", {
  position: "absolute",
  width: "1px",
  height: "1px",
  padding: 0,
  margin: "-1px",
  overflow: "hidden",
  clip: "rect(0, 0, 0, 0)",
  whiteSpace: "nowrap",
  border: 0,
});

globalStyle(".admin-page, .ontology-page", {
  display: "flex",
  flex: "1 1 auto",
  flexDirection: "column",
  gap: "0.5rem",
  minHeight: 0,
  overflow: "hidden",
});

globalStyle(".admin-page__header, .ontology-header", {
  display: "flex",
  flex: "0 0 auto",
  gap: "0.75rem",
  alignItems: "center",
  justifyContent: "space-between",
  minHeight: 0,
});

globalStyle(".admin-page__title, .ontology-header__title", {
  margin: 0,
  fontSize: "clamp(1.15rem, 1.6vw, 1.4rem)",
  lineHeight: "1.1",
});

globalStyle(".admin-page__panel", {
  display: "grid",
  flex: "1 1 auto",
  minHeight: 0,
  overflow: "hidden",
});

globalStyle(".admin-page__body", {
  display: "flex",
  flex: "1 1 auto",
  flexDirection: "column",
  minHeight: 0,
  overflowY: "auto",
});

globalStyle(".admin-page__code-block", {
  flex: "1 1 auto",
  minHeight: 0,
});

globalStyle(".ontology-header__copy", {
  display: "flex",
  gap: "0.6rem",
  alignItems: "baseline",
  minWidth: 0,
});

globalStyle(".ontology-header__title", {
  whiteSpace: "nowrap",
});

globalStyle(".ontology-header__meta", {
  display: "flex",
  flexWrap: "nowrap",
  gap: "0.5rem",
  alignItems: "center",
  whiteSpace: "nowrap",
});

globalStyle(".ontology-layout", {
  display: "grid",
  gap: "1rem",
  minHeight: 0,
  overflow: "hidden",
});

globalStyle(".ontology-pane", {
  display: "grid",
  gap: "0.75rem",
  minHeight: 0,
  background: "linear-gradient(180deg, rgba(6, 27, 49, 0.9), rgba(4, 21, 39, 0.88))",
});

globalStyle(".ontology-pane__header", {
  alignContent: "start",
});

globalStyle(".ontology-pane__body", {
  minHeight: 0,
  overflow: "hidden",
});

globalStyle(".ontology-sidebar", {
  alignContent: "start",
});

globalStyle(".ontology-entity-list", {
  display: "grid",
  gap: "0.5rem",
});

globalStyle(".ontology-entity", {
  display: "block",
  width: "100%",
  padding: "0.45rem 0.1rem 0.45rem 0.85rem",
  border: 0,
  borderLeft: "2px solid transparent",
  borderRadius: 0,
  background: "transparent",
  textAlign: "left",
});

globalStyle(".ontology-entity:hover", {
  borderLeftColor: "rgba(139, 245, 212, 0.45)",
  background: "rgba(139, 245, 212, 0.05)",
});

globalStyle(".ontology-entity--selected", {
  borderLeftColor: "rgba(139, 245, 212, 0.72)",
  background: "rgba(139, 245, 212, 0.1)",
});

globalStyle(".ontology-entity__row", {
  display: "flex",
  gap: "0.5rem",
  alignItems: "baseline",
  justifyContent: "flex-start",
  whiteSpace: "nowrap",
});

globalStyle(".ontology-entity__kind", {
  color: "var(--accent)",
  flex: "0 0 auto",
  fontSize: "0.72rem",
  fontWeight: "700",
  letterSpacing: "0.04em",
});

globalStyle(".ontology-entity__name", {
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  fontSize: "1rem",
});

globalStyle(".ontology-detail", {
  flex: "1 1 auto",
  minHeight: 0,
  overflowY: "auto",
  paddingRight: "0.25rem",
});

globalStyle(".ontology-detail__header", {
  display: "grid",
  gap: "0.35rem",
  justifyItems: "start",
  textAlign: "left",
});

globalStyle(".ontology-detail__title", {
  margin: 0,
  fontSize: "clamp(1.35rem, 2.4vw, 1.85rem)",
});

globalStyle(".ontology-detail__meta", {
  margin: "0.2rem 0 0",
  color: "var(--muted)",
  fontFamily: '"IBM Plex Mono", "SFMono-Regular", monospace',
  fontSize: "0.84rem",
});

globalStyle(".ontology-detail__hero", {
  paddingBottom: "0.25rem",
});

globalStyle(".ontology-section", {
  display: "grid",
  gap: "0.65rem",
});

globalStyle(".ontology-section--spaced", {
  paddingTop: "0.5rem",
});

globalStyle(".ontology-section h3", {
  margin: 0,
  fontSize: "1rem",
});

globalStyle(".ontology-inline-links", {
  display: "flex",
  flexWrap: "wrap",
  gap: "0.35rem 0.6rem",
  alignItems: "center",
});

globalStyle(".nosqo-table", {
  width: "100%",
  borderCollapse: "collapse",
  tableLayout: "fixed",
  fontSize: "0.92rem",
});

globalStyle(".nosqo-table th", {
  padding: "0 0 0.3rem",
  color: "var(--muted)",
  fontSize: "0.74rem",
  fontWeight: "700",
  letterSpacing: "0.04em",
  textAlign: "left",
  textTransform: "uppercase",
});

globalStyle(".nosqo-table td", {
  padding: "0.2rem 0.75rem 0.2rem 0",
  verticalAlign: "top",
  textAlign: "left",
});

globalStyle(".nosqo-table th:last-child, .nosqo-table td:last-child", {
  paddingRight: 0,
});

globalStyle(".ontology-reference-table td:first-child", {
  width: "20%",
});

globalStyle(".ontology-inline-link", {
  padding: 0,
  border: 0,
  background: "transparent",
  color: "var(--accent)",
  fontWeight: "600",
  textDecoration: "underline",
  textUnderlineOffset: "0.15em",
});

globalStyle(".ontology-inline-link:hover", {
  color: "#f8fafc",
  borderColor: "transparent",
  background: "transparent",
  transform: "none",
});

globalStyle(".ontology-inline-link--secondary", {
  color: "rgba(183, 247, 228, 0.82)",
  fontWeight: "500",
});

globalStyle(".ontology-tag-list", {
  display: "flex",
  flexWrap: "wrap",
  gap: "0.5rem",
});

globalStyle(".ontology-tag", {
  display: "inline-flex",
  alignItems: "center",
  padding: "0.35rem 0.6rem",
  border: "1px solid var(--border-color)",
  borderRadius: "999px",
  background: "rgba(12, 51, 90, 0.42)",
  color: "#f8fafc",
  fontSize: "0.85rem",
});

globalStyle(".app-shell", {
  width: "min(1120px, calc(100vw - 2rem))",
  margin: "0 auto",
  padding: "1.5rem 0 3rem",
});

globalStyle(".hero", {
  display: "grid",
  gap: "1rem",
  alignItems: "end",
  marginBottom: "1.5rem",
});

globalStyle(".hero__copy h1", {
  margin: 0,
  maxWidth: "16ch",
  fontSize: "clamp(2.5rem, 6vw, 5rem)",
  lineHeight: "0.95",
});

globalStyle(".hero__lede", {
  margin: "1rem 0 0",
  maxWidth: "58ch",
  color: "var(--muted)",
});

globalStyle(".feature-grid, .filters-grid, .ontology-relationship-grid", {
  "@media": {
    "(min-width: 720px)": {
      gridTemplateColumns: "repeat(2, minmax(0, 1fr))",
    },
  },
});

globalStyle(".filters-grid .field:last-of-type, .filters-grid .toolbar--filters", {
  "@media": {
    "(min-width: 720px)": {
      gridColumn: "span 1",
    },
  },
});

globalStyle(".admin-page, .ontology-page", {
  "@media": {
    "(min-width: 960px)": {
      height: "100%",
    },
  },
});

globalStyle(".ontology-layout", {
  "@media": {
    "(min-width: 960px)": {
      flex: "1 1 auto",
      height: "100%",
      gridTemplateColumns: "minmax(18rem, 22rem) minmax(0, 1fr)",
      alignItems: "stretch",
    },
  },
});

globalStyle(".ontology-pane", {
  "@media": {
    "(min-width: 960px)": {
      height: "100%",
      gridTemplateRows: "auto minmax(0, 1fr)",
      overflow: "hidden",
    },
  },
});

globalStyle(".ontology-pane__body", {
  "@media": {
    "(min-width: 960px)": {
      display: "flex",
      flexDirection: "column",
      minHeight: 0,
      overflow: "hidden",
      paddingRight: "0.25rem",
    },
  },
});

globalStyle(".ontology-entity-list", {
  "@media": {
    "(min-width: 960px)": {
      flex: "1 1 auto",
      minHeight: 0,
      overflowY: "auto",
      paddingRight: "0.25rem",
    },
  },
});

globalStyle(".ontology-sidebar", {
  "@media": {
    "(min-width: 960px)": {
      minHeight: 0,
    },
  },
});

globalStyle(".ontology-sidebar .ontology-pane__body, .ontology-detail-pane .ontology-pane__body", {
  "@media": {
    "(min-width: 960px)": {
      minHeight: 0,
    },
  },
});

globalStyle(".ontology-sidebar .empty-state, .ontology-detail-pane .empty-state", {
  "@media": {
    "(min-width: 960px)": {
      margin: "auto 0",
    },
  },
});

globalStyle(".filters-grid", {
  "@media": {
    "(min-width: 960px)": {
      gridTemplateColumns: "repeat(4, minmax(0, 1fr))",
    },
  },
});

globalStyle(".nosqo-shell", {
  "@media": {
    "(max-width: 47.99rem)": {
      gridTemplateRows: "44px minmax(0, 1fr)",
    },
  },
});

globalStyle(".shell-burger", {
  "@media": {
    "(max-width: 47.99rem)": {
      display: "inline-flex",
      alignItems: "center",
      justifyContent: "center",
    },
  },
});

globalStyle(".shell-body", {
  "@media": {
    "(max-width: 47.99rem)": {
      gridTemplateColumns: "minmax(0, 1fr)",
    },
  },
});

globalStyle(".shell-navbar", {
  "@media": {
    "(max-width: 47.99rem)": {
      position: "fixed",
      top: "44px",
      bottom: 0,
      left: 0,
      width: "min(18rem, 85vw)",
      transform: "translateX(-100%)",
      transition: "transform 160ms ease",
      zIndex: "20",
    },
  },
});

globalStyle(".shell-navbar--open", {
  "@media": {
    "(max-width: 47.99rem)": {
      transform: "translateX(0)",
    },
  },
});

globalStyle(".shell-main__viewport, .shell-main__content", {
  "@media": {
    "(max-width: 47.99rem)": {
      width: "100%",
    },
  },
});

globalStyle(".shell-main__content", {
  "@media": {
    "(max-width: 47.99rem)": {
      padding: "1rem",
    },
  },
});

globalStyle(".shell-backdrop", {
  "@media": {
    "(max-width: 47.99rem)": {
      display: "block",
      position: "fixed",
      inset: "44px 0 0 0",
      border: 0,
      background: "rgba(2, 8, 23, 0.45)",
      zIndex: "10",
    },
  },
});

globalStyle(".breadcrumb-list", {
  "@media": {
    "(max-width: 47.99rem)": {
      display: "none",
    },
  },
});
