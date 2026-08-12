import { useState } from "react";
import "./App.css";
import Navbar from "./components/Navbar";
import type { NavItem } from "./components/Navbar";
import ProductosView from "./views/ProductosView";
import CategoriasView from "./views/CategoriasView";
import MarcasView from "./views/MarcasView";

function App() {
  const [view, setView] = useState<NavItem>("productos");

  return (
    <>
      <Navbar active={view} onChange={setView} />
      {view === "productos" && <ProductosView />}
      {view === "categorias" && <CategoriasView />}
      {view === "marcas" && <MarcasView />}
    </>
  );
}

export default App;
