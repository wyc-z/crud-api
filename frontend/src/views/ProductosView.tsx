import { useEffect, useState } from "react";
import ProductForm from "../components/forms/ProductForm";
import type { ProductFormData } from "../components/forms/ProductForm";
import type { NameDescriptionCreateData } from "../components/forms/NameDescriptionCreate";
import ProductCard from "../components/cards/ProductCard";
import type { Brand, Category, Product } from "../types";
import {
  createBrand,
  createCategory,
  createProduct,
  deleteProduct,
  getBrands,
  getCategories,
  getProducts,
  updateProduct,
} from "../services/api";

export default function ProductosView() {
  const [products, setProducts] = useState<Product[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<Product | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    Promise.all([getProducts(), getCategories(), getBrands()])
      .then(([productsData, categoriesData, brandsData]) => {
        if (cancelled) return;
        setProducts(productsData);
        setCategories(categoriesData);
        setBrands(brandsData);
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : "Error de red");
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const showForm = creating || editing !== null;

  const handleSubmit = async (data: ProductFormData) => {
    setError(null);
    try {
      if (editing) {
        const updated = await updateProduct(editing.id, data);
        setProducts((prev) =>
          prev.map((item) => (item.id === updated.id ? updated : item)),
        );
      } else {
        const created = await createProduct(data);
        setProducts((prev) => [...prev, created]);
      }
      setCreating(false);
      setEditing(null);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Error al guardar el producto",
      );
    }
  };

  const handleDelete = async (product: Product) => {
    setError(null);
    try {
      await deleteProduct(product.id);
      setProducts((prev) => prev.filter((item) => item.id !== product.id));
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Error al eliminar el producto",
      );
    }
  };

  const handleCreateCategory = async (data: NameDescriptionCreateData) => {
    const created = await createCategory(data);
    setCategories((prev) => [...prev, created]);
    return created;
  };

  const handleCreateBrand = async (data: NameDescriptionCreateData) => {
    const created = await createBrand(data);
    setBrands((prev) => [...prev, created]);
    return created;
  };

  if (loading) {
    return (
      <div className="container py-4">
        <p className="text-center text-body-secondary py-5 mb-0">
          Cargando productos…
        </p>
      </div>
    );
  }

  if (showForm) {
    return (
      <div className="container py-4">
        {error ? (
          <div className="alert alert-danger" role="alert">
            {error}
          </div>
        ) : null}
        <ProductForm
          key={editing ? `edit-${editing.id}` : "new"}
          title={editing ? "Editar producto" : "Nuevo producto"}
          submitLabel={editing ? "Guardar cambios" : "Guardar producto"}
          initialValues={editing ?? undefined}
          categories={categories}
          brands={brands}
          onCreateCategory={handleCreateCategory}
          onCreateBrand={handleCreateBrand}
          onSubmit={handleSubmit}
        />
      </div>
    );
  }

  return (
    <div className="container py-4">
      <header className="d-flex align-items-center justify-content-between mb-4">
        <h1 className="h3 mb-0">Productos</h1>
        <button
          type="button"
          className="btn btn-primary"
          onClick={() => setCreating(true)}
        >
          Nuevo producto
        </button>
      </header>

      {error ? (
        <div className="alert alert-danger" role="alert">
          {error}
        </div>
      ) : null}

      {products.length === 0 ? (
        <p className="text-center text-body-secondary py-5 mb-0">
          No hay productos todavía.
        </p>
      ) : (
        <div className="row row-cols-1 row-cols-md-2 row-cols-lg-3 g-3">
          {products.map((product) => (
            <div key={product.id} className="col">
              <ProductCard
                product={product}
                onEdit={() => setEditing(product)}
                onDelete={() => handleDelete(product)}
              />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
