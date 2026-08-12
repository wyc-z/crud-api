import { useState } from "react";
import type { ChangeEvent } from "react";
import FormField from "./FormField";

export type NameDescriptionCreateData = {
  name: string;
  description: string;
};

type NameDescriptionCreateProps = {
  title: string;
  submitLabel?: string;
  error?: string | null;
  onSubmit: (data: NameDescriptionCreateData) => Promise<void> | void;
};

export default function NameDescriptionCreate({
  title,
  submitLabel = "Crear",
  error,
  onSubmit,
}: NameDescriptionCreateProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [nameError, setNameError] = useState<string | undefined>();
  const [descriptionError, setDescriptionError] = useState<
    string | undefined
  >();
  const [submitting, setSubmitting] = useState(false);

  const handleNameChange = (event: ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
    if (nameError) {
      setNameError(undefined);
    }
  };

  const handleSubmit = async () => {
    const trimmedName = name.trim();
    if (trimmedName === "") {
      setNameError("El nombre es obligatorio.");
      return;
    }
    if (trimmedName.length < 3) {
      setNameError("El nombre debe tener al menos 3 caracteres.");
      return;
    }
    if (trimmedName.length > 20) {
      setNameError("El nombre no puede superar los 20 caracteres.");
      return;
    }
    const trimmedDescription = description.trim();
    if (trimmedDescription.length > 60) {
      setDescriptionError(
        "La descripción no puede superar los 60 caracteres.",
      );
      return;
    }
    setNameError(undefined);
    setDescriptionError(undefined);
    setSubmitting(true);
    try {
      await onSubmit({ name: trimmedName, description: trimmedDescription });
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="border rounded p-2 mt-2 bg-body-tertiary">
      <p className="small text-body-secondary mb-2">{title}</p>

      <FormField label="Nombre" error={nameError} required small className="mb-2">
        <input
          className="form-control form-control-sm"
          value={name}
          onChange={handleNameChange}
          placeholder="Ej. Bebidas"
        />
      </FormField>

      <FormField label="Descripción" error={descriptionError} small className="mb-2">
        <input
          className="form-control form-control-sm"
          value={description}
          onChange={(event) => {
            setDescription(event.target.value);
            if (descriptionError) {
              setDescriptionError(undefined);
            }
          }}
          placeholder="Descripción opcional"
        />
      </FormField>

      {error ? (
        <div className="alert alert-danger py-1 px-2 small mb-2" role="alert">
          {error}
        </div>
      ) : null}

      <button
        type="button"
        className="btn btn-primary btn-sm"
        onClick={handleSubmit}
        disabled={submitting}
      >
        {submitting ? "Creando…" : submitLabel}
      </button>
    </div>
  );
}