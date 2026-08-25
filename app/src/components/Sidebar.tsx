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