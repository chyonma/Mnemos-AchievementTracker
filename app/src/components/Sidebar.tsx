import { NavLink } from "react-router-dom";

export function Sidebar() {
  const linkStyle = ({ isActive }: { isActive: boolean }) => ({
    display: "block",
    padding: "8px 12px",
    borderRadius: "var(--radius)",
    color: isActive ? "var(--text-primary)" : "var(--text-secondary)",
    background: isActive ? "var(--surface-2)" : "transparent",
    textDecoration: "none",
    fontSize: "14px",
  });

  return (
    <nav style={{ width: 160, padding: "16px 10px", borderRight: "1px solid var(--border)" }}>
      <p style={{ fontWeight: 500, padding: "0 12px 14px" }}>Mnemos</p>
      <NavLink to="/" end style={linkStyle}>Home</NavLink>
      <NavLink to="/library" style={linkStyle}>Library</NavLink>
      <NavLink to="/achievements" style={linkStyle}>Achievements</NavLink>
      <NavLink to="/settings" style={linkStyle}>Settings</NavLink>
    </nav>
  );
}