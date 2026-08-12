import NameDescriptionView from "../components/NameDescriptionView";
import {
  createBrand,
  deleteBrand,
  getBrands,
  updateBrand,
} from "../services/api";

export default function MarcasView() {
  return (
    <NameDescriptionView
      title="Marcas"
      newButtonLabel="Nueva marca"
      formTitle="Nueva marca"
      editTitle="Editar marca"
      loadItems={getBrands}
      createItem={createBrand}
      updateItem={updateBrand}
      deleteItem={deleteBrand}
    />
  );
}
