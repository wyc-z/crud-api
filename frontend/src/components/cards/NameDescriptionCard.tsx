type NameDescriptionCardProps = {
  item: { name: string; description?: string | null };
  onEdit: () => void;
  onDelete: () => void;
};

export default function NameDescriptionCard({
  item,
  onEdit,
  onDelete,
}: NameDescriptionCardProps) {
  return (
    <article className="card h-100">
      <div className="card-body d-flex justify-content-between gap-3">
        <div>
          <h2 className="card-title h6 mb-1">{item.name}</h2>
          {item.description ? (
            <p className="card-text small text-body-secondary mb-0">
              {item.description}
            </p>
          ) : null}
        </div>
        <div className="d-flex flex-column gap-1">
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={onEdit}
            aria-label={`Editar ${item.name}`}
            title="Editar"
          >
            <i className="bi bi-pencil" aria-hidden="true" />
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-danger"
            onClick={onDelete}
            aria-label={`Eliminar ${item.name}`}
            title="Eliminar"
          >
            <i className="bi bi-trash" aria-hidden="true" />
          </button>
        </div>
      </div>
    </article>
  );
}
