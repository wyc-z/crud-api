export type Product = {
  id: number;
  name: string;
  brand_id?: number | null;
  category_id: number;
  cost: string;
  price: string;
  stock: number;
  sku: string;
  description?: string | null;
};

export type ProductPayload = {
  name: string;
  brand_id?: number | null;
  category_id: number;
  cost: string;
  price: string;
  stock: number;
  sku: string;
  description?: string | null;
};

export type NameDescriptionItem = {
  id: number;
  name: string;
  description?: string | null;
};

export type NameDescriptionPayload = {
  name: string;
  description?: string | null;
};

export type Category = NameDescriptionItem;

export type Brand = NameDescriptionItem;
