import { useEffect, useState } from "react";
import NameDescriptionForm from "./forms/NameDescriptionForm";
import type { NameDescriptionData } from "./forms/NameDescriptionForm";
import NameDescriptionCard from "./cards/NameDescriptionCard";
import type { NameDescriptionItem } from "../types";

type NameDescriptionViewProps = {
  title: string;
  newButtonLabel: string;
  formTitle: string;
  editTitle: string;
  loadItems: () => Promise<NameDescriptionItem[]>;
  createItem: (data: NameDescriptionData) => Promise<NameDescriptionItem>;
  updateItem: (
    id: number,
    data: NameDescriptionData,
  ) => Promise<NameDescriptionItem>;
  deleteItem: (id: number) => Promise<void>;
};

export default function NameDescriptionView({
  title,
  newButtonLabel,
  formTitle,
  editTitle,
  loadItems,
  createItem,
  updateItem,
  deleteItem,
}: NameDescriptionViewProps) {
  const [items, setItems] = useState<NameDescriptionItem[]>([]);
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<NameDescriptionItem | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    loadItems()
      .then((data) => {
        if (!cancelled) setItems(data);
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
  }, [loadItems]);

  const showForm = creating || editing !== null;

  const handleSubmit = async (data: NameDescriptionData) => {
    setError(null);
    try {
      if (editing) {
        const updated = await updateItem(editing.id, data);
        setItems((prev) =>
          prev.map((item) => (item.id === updated.id ? updated : item)),
        );
      } else {
        const created = await createItem(data);
        setItems((prev) => [...prev, created]);
      }
      setCreating(false);
      setEditing(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Error al guardar");
    }
  };

  const handleDelete = async (item: NameDescriptionItem) => {
    setError(null);
    try {
      await deleteItem(item.id);
      setItems((prev) => prev.filter((entry) => entry.id !== item.id));
    } catch (err) {
      setError(err instanceof Error ? err.message : "Error al eliminar");
    }
  };

  if (loading) {
    return (
      <div className="container py-4">
        <p className="text-center text-body-secondary py-5 mb-0">
          Cargando {title.toLowerCase()}…
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
        <NameDescriptionForm
          key={editing ? `edit-${editing.id}` : "new"}
          title={editing ? editTitle : formTitle}
          submitLabel={editing ? "Guardar cambios" : "Guardar"}
          initialValues={editing ?? undefined}
          onSubmit={handleSubmit}
        />
      </div>
    );
  }

  return (
    <div className="container py-4">
      <header className="d-flex align-items-center justify-content-between mb-4">
        <h1 className="h3 mb-0">{title}</h1>
        <button
          type="button"
          className="btn btn-primary"
          onClick={() => setCreating(true)}
        >
          {newButtonLabel}
        </button>
      </header>

      {error ? (
        <div className="alert alert-danger" role="alert">
          {error}
        </div>
      ) : null}

      {items.length === 0 ? (
        <p className="text-center text-body-secondary py-5 mb-0">
          No hay {title.toLowerCase()} todavía.
        </p>
      ) : (
        <div className="row row-cols-1 row-cols-md-2 row-cols-lg-3 g-3">
          {items.map((item) => (
            <div key={item.id} className="col">
              <NameDescriptionCard
                item={item}
                onEdit={() => setEditing(item)}
                onDelete={() => handleDelete(item)}
              />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
