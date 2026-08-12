type ProductCardProps = {
  product: { name: string; sku: string; price: string; stock: number };
  onEdit: () => void;
  onDelete: () => void;
};

export default function ProductCard({
  product,
  onEdit,
  onDelete,
}: ProductCardProps) {
  return (
    <article className="card h-100">
      <div className="card-body d-flex justify-content-between gap-3">
        <div>
          <h2 className="card-title h6 mb-2">{product.name}</h2>
          <dl className="row mb-0 small text-body-secondary">
            <dt className="col-4 mb-0">SKU</dt>
            <dd className="col-8 mb-0">{product.sku}</dd>
            <dt className="col-4 mb-0">Precio</dt>
            <dd className="col-8 mb-0">${Number(product.price).toFixed(2)}</dd>
            <dt className="col-4 mb-0">Stock</dt>
            <dd className="col-8 mb-0">{product.stock}</dd>
          </dl>
        </div>
        <div className="d-flex flex-column gap-1">
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={onEdit}
            aria-label={`Editar ${product.name}`}
            title="Editar"
          >
            <i className="bi bi-pencil" aria-hidden="true" />
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-danger"
            onClick={onDelete}
            aria-label={`Eliminar ${product.name}`}
            title="Eliminar"
          >
            <i className="bi bi-trash" aria-hidden="true" />
          </button>
        </div>
      </div>
    </article>
  );
}
