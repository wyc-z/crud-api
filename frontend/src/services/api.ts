import type {
  Brand,
  Category,
  NameDescriptionPayload,
  Product,
  ProductPayload,
} from "../types";

const BASE = "/api";

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const resp = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!resp.ok) {
    let message = `Error en la solicitud: ${resp.status}`;
    try {
      const body = (await resp.json()) as { error?: unknown };
      if (typeof body?.error === "string" && body.error.trim() !== "") {
        message = body.error;
      }
    } catch {
      // responde sin cuerpo JSON; se conserva el mensaje genérico
    }
    throw new Error(message);
  }
  if (resp.status === 204) return undefined as T;
  return resp.json();
}

export const getProducts = (): Promise<Product[]> => request("/products");

export const getProduct = (id: number): Promise<Product> =>
  request(`/products/${id}`);

export const getProductBySku = (sku: string): Promise<Product> =>
  request(`/products/sku/${sku}`);

export const getProductsByCategory = (categoryId: number): Promise<Product[]> =>
  request(`/products/category/${categoryId}`);

export const createProduct = (payload: ProductPayload): Promise<Product> =>
  request("/products", { method: "POST", body: JSON.stringify(payload) });

export const updateProduct = (
  id: number,
  payload: ProductPayload,
): Promise<Product> =>
  request(`/products/${id}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });

export const deleteProduct = (id: number): Promise<void> =>
  request(`/products/${id}`, { method: "DELETE" });

export const getCategories = (): Promise<Category[]> => request("/categories");

export const createCategory = (
  payload: NameDescriptionPayload,
): Promise<Category> =>
  request("/categories", { method: "POST", body: JSON.stringify(payload) });

export const updateCategory = (
  id: number,
  payload: NameDescriptionPayload,
): Promise<Category> =>
  request(`/categories/${id}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });

export const deleteCategory = (id: number): Promise<void> =>
  request(`/categories/${id}`, { method: "DELETE" });

export const getBrands = (): Promise<Brand[]> => request("/brands");

export const createBrand = (payload: NameDescriptionPayload): Promise<Brand> =>
  request("/brands", { method: "POST", body: JSON.stringify(payload) });

export const updateBrand = (
  id: number,
  payload: NameDescriptionPayload,
): Promise<Brand> =>
  request(`/brands/${id}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });

export const deleteBrand = (id: number): Promise<void> =>
  request(`/brands/${id}`, { method: "DELETE" });
