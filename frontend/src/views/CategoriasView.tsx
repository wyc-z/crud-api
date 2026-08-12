import NameDescriptionView from "../components/NameDescriptionView";
import {
  createCategory,
  deleteCategory,
  getCategories,
  updateCategory,
} from "../services/api";

export default function CategoriasView() {
  return (
    <NameDescriptionView
      title="Categorías"
      newButtonLabel="Nueva categoría"
      formTitle="Nueva categoría"
      editTitle="Editar categoría"
      loadItems={getCategories}
      createItem={createCategory}
      updateItem={updateCategory}
      deleteItem={deleteCategory}
    />
  );
}
