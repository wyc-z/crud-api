export type FormValues = {
  name: string;
  brand_id: string;
  category_id: string;
  cost: string;
  price: string;
  stock: string;
  sku: string;
  description: string;
};

export type ValidationErrors = Partial<Record<keyof FormValues, string>>;

export const CREATE_NEW_VALUE = "__new__";

export function validateProductForm(values: FormValues): ValidationErrors {
  const errors: ValidationErrors = {};

  const name = values.name.trim();
  if (name === "") {
    errors.name = "El nombre es obligatorio.";
  } else if (name.length < 3) {
    errors.name = "El nombre debe tener al menos 3 caracteres.";
  } else if (name.length > 50) {
    errors.name = "El nombre no puede superar los 50 caracteres.";
  }

  const sku = values.sku.trim();
  if (sku === "") {
    errors.sku = "El SKU es obligatorio.";
  } else if (sku.length < 3) {
    errors.sku = "El SKU debe tener al menos 3 caracteres.";
  } else if (sku.length > 50) {
    errors.sku = "El SKU no puede superar los 50 caracteres.";
  }

  if (values.category_id === "" || values.category_id === CREATE_NEW_VALUE) {
    errors.category_id = "Selecciona o crea una categoría.";
  }

  const cost = Number(values.cost);
  const price = Number(values.price);
  if (values.price.trim() === "" || Number.isNaN(price)) {
    errors.price = "Ingresa un precio válido.";
  } else if (price <= 0) {
    errors.price = "El precio debe ser mayor a 0.";
  } else if (!Number.isNaN(cost) && price <= cost) {
    errors.price = "El precio debe ser mayor al costo.";
  }

  const stock = Number(values.stock);
  if (values.stock.trim() !== "" && (Number.isNaN(stock) || stock < 0)) {
    errors.stock = "El stock no puede ser negativo.";
  }

  const description = values.description.trim();
  if (description.length > 120) {
    errors.description = "La descripción no puede superar los 120 caracteres.";
  }

  return errors;
}
