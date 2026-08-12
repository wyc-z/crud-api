export type NavItem = "productos" | "categorias" | "marcas";

type NavbarProps = {
  active: NavItem;
  onChange: (item: NavItem) => void;
};

const NAV_ITEMS: { id: NavItem; label: string }[] = [
  { id: "productos", label: "Productos" },
  { id: "categorias", label: "Categorías" },
  { id: "marcas", label: "Marcas" },
];

export default function Navbar({ active, onChange }: NavbarProps) {
  return (
    <nav
      className="nav nav-pills py-3 px-3 bg-body border-bottom justify-content-center"
      aria-label="Secciones"
    >
      {NAV_ITEMS.map((item) => (
        <button
          key={item.id}
          type="button"
          className={`nav-link px-3 ${active === item.id ? "active fw-semibold" : ""}`}
          aria-current={active === item.id ? "page" : undefined}
          onClick={() => onChange(item.id)}
        >
          {item.label}
        </button>
      ))}
    </nav>
  );
}