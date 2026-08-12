import { useState } from "react";
import type { ChangeEvent, SubmitEvent } from "react";
import FormField from "./FormField";
import NameDescriptionCreate from "./NameDescriptionCreate";
import type { NameDescriptionCreateData } from "./NameDescriptionCreate";
import { CREATE_NEW_VALUE, validateProductForm } from "./validation";
import type { FormValues } from "./validation";
import type { Brand, Category } from "../../types";

export type ProductFormData = {
  name: string;
  brand_id?: number | null;
  category_id: number;
  cost: string;
  price: string;
  stock: number;
  sku: string;
  description?: string | null;
};

type ValidationErrors = Partial<Record<keyof FormValues, string>>;

type ProductFormProps = {
  onSubmit: (data: ProductFormData) => Promise<void> | void;
  title?: string;
  submitLabel?: string;
  initialValues?: Partial<ProductFormData>;
  categories: Category[];
  brands: Brand[];
  onCreateCategory?: (data: NameDescriptionCreateData) => Promise<Category>;
  onCreateBrand?: (data: NameDescriptionCreateData) => Promise<Brand>;
};

type FieldElement = HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

const INITIAL_VALUES: FormValues = {
  name: "",
  brand_id: "",
  category_id: "",
  cost: "0",
  price: "0",
  stock: "0",
  sku: "",
  description: "",
};

function toFormValues(data: Partial<ProductFormData>): FormValues {
  return {
    name: data.name ?? "",
    brand_id: data.brand_id != null ? String(data.brand_id) : "",
    category_id: data.category_id !== undefined ? String(data.category_id) : "",
    cost: data.cost !== undefined ? String(data.cost) : "0",
    price: data.price !== undefined ? String(data.price) : "0",
    stock: data.stock !== undefined ? String(data.stock) : "0",
    sku: data.sku ?? "",
    description: data.description ?? "",
  };
}

function toProductFormData(values: FormValues): ProductFormData {
  return {
    name: values.name.trim(),
    brand_id:
      values.brand_id && values.brand_id !== CREATE_NEW_VALUE
        ? Number(values.brand_id)
        : undefined,
    category_id: Number(values.category_id),
    cost: values.cost.trim() || "0",
    price: values.price.trim(),
    stock: Number(values.stock) || 0,
    sku: values.sku.trim(),
    description: values.description.trim() || undefined,
  };
}

export default function ProductForm({
  onSubmit,
  title = "Crear producto",
  submitLabel = "Guardar producto",
  initialValues,
  categories,
  brands,
  onCreateCategory,
  onCreateBrand,
}: ProductFormProps) {
  const [values, setValues] = useState<FormValues>(() =>
    initialValues ? toFormValues(initialValues) : INITIAL_VALUES,
  );
  const [errors, setErrors] = useState<ValidationErrors>({});
  const [submitting, setSubmitting] = useState(false);
  const [categoryCreateError, setCategoryCreateError] = useState<string | null>(
    null,
  );
  const [brandCreateError, setBrandCreateError] = useState<string | null>(null);

  const handleChange =
    (field: keyof FormValues) => (event: ChangeEvent<FieldElement>) => {
      setValues((prev) => ({ ...prev, [field]: event.target.value }));
      if (errors[field]) {
        setErrors((prev) => ({ ...prev, [field]: undefined }));
      }
    };

  const handleCreateCategory = async (data: NameDescriptionCreateData) => {
    if (!onCreateCategory) return;
    setCategoryCreateError(null);
    try {
      const created = await onCreateCategory(data);
      setValues((prev) => ({ ...prev, category_id: String(created.id) }));
      setErrors((prev) => ({ ...prev, category_id: undefined }));
    } catch (err) {
      setCategoryCreateError(
        err instanceof Error ? err.message : "Error al crear la categoría",
      );
    }
  };

  const handleCreateBrand = async (data: NameDescriptionCreateData) => {
    if (!onCreateBrand) return;
    setBrandCreateError(null);
    try {
      const created = await onCreateBrand(data);
      setValues((prev) => ({ ...prev, brand_id: String(created.id) }));
    } catch (err) {
      setBrandCreateError(
        err instanceof Error ? err.message : "Error al crear la marca",
      );
    }
  };

  const handleSubmit = async (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    const nextErrors = validateProductForm(values);
    setErrors(nextErrors);
    if (Object.keys(nextErrors).length > 0) {
      return;
    }
    setSubmitting(true);
    try {
      await onSubmit(toProductFormData(values));
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form
      className="card"
      onSubmit={handleSubmit}
      noValidate
      aria-label={title}
    >
      <header className="card-header">
        <h1 className="h5 mb-0">{title}</h1>
        <p className="text-body-secondary small mb-0">
          Completa los campos para guardar el producto.
        </p>
      </header>

      <div className="card-body">
        <div className="row g-3">
          <div className="col-12 col-md-6">
            <FormField label="Nombre" error={errors.name} required>
              <input
                className="form-control"
                value={values.name}
                onChange={handleChange("name")}
                placeholder="Ej. S25 Ultra"
              />
            </FormField>
          </div>

          <div className="col-12 col-md-6">
            <FormField label="SKU" error={errors.sku} required>
              <input
                className="form-control"
                value={values.sku}
                onChange={handleChange("sku")}
                placeholder="Ej. SM-S938"
              />
            </FormField>
          </div>

          <div className="col-12 col-md-6">
            <FormField label="Categoría" error={errors.category_id} required>
              <select
                className="form-select"
                value={values.category_id}
                onChange={handleChange("category_id")}
              >
                <option value="">Seleccionar</option>
                {categories.map((category) => (
                  <option key={category.id} value={category.id}>
                    {category.name}
                  </option>
                ))}
                {onCreateCategory ? (
                  <option value={CREATE_NEW_VALUE}>+ Nueva categoría…</option>
                ) : null}
              </select>
            </FormField>
            {values.category_id === CREATE_NEW_VALUE ? (
              <NameDescriptionCreate
                title="Nueva categoría"
                submitLabel="Crear categoría"
                error={categoryCreateError}
                onSubmit={handleCreateCategory}
              />
            ) : null}
          </div>

          <div className="col-12 col-md-6">
            <FormField label="Marca">
              <select
                className="form-select"
                value={values.brand_id}
                onChange={handleChange("brand_id")}
              >
                <option value="">Seleccionar</option>
                {brands.map((brand) => (
                  <option key={brand.id} value={brand.id}>
                    {brand.name}
                  </option>
                ))}
                {onCreateBrand ? (
                  <option value={CREATE_NEW_VALUE}>+ Nueva marca…</option>
                ) : null}
              </select>
            </FormField>
            {values.brand_id === CREATE_NEW_VALUE ? (
              <NameDescriptionCreate
                title="Nueva marca"
                submitLabel="Crear marca"
                error={brandCreateError}
                onSubmit={handleCreateBrand}
              />
            ) : null}
          </div>

          <div className="col-12 col-md-6">
            <FormField label="Costo">
              <input
                className="form-control"
                type="number"
                min={0}
                step="0.01"
                value={values.cost}
                onChange={handleChange("cost")}
              />
            </FormField>
          </div>

          <div className="col-12 col-md-6">
            <FormField label="Precio" error={errors.price} required>
              <input
                className="form-control"
                type="number"
                min={0}
                step="0.01"
                value={values.price}
                onChange={handleChange("price")}
              />
            </FormField>
          </div>

          <div className="col-12 col-md-6">
            <FormField label="Stock">
              <input
                className="form-control"
                type="number"
                min={0}
                step="1"
                value={values.stock}
                onChange={handleChange("stock")}
              />
            </FormField>
          </div>

          <div className="col-12">
            <FormField label="Descripción">
              <textarea
                className="form-control"
                rows={4}
                value={values.description}
                onChange={handleChange("description")}
                placeholder="Descripción opcional del producto"
              />
            </FormField>
          </div>
        </div>
      </div>

      <footer className="card-footer d-flex justify-content-end">
        <button type="submit" className="btn btn-primary" disabled={submitting}>
          {submitting ? "Guardando…" : submitLabel}
        </button>
      </footer>
    </form>
  );
}
