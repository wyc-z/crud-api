import { useState } from "react";
import type { ChangeEvent, SubmitEvent } from "react";
import FormField from "./FormField";

export type NameDescriptionData = {
  name: string;
  description?: string | null;
};

type NameDescriptionFormProps = {
  title: string;
  submitLabel?: string;
  initialValues?: Partial<NameDescriptionData>;
  onSubmit: (data: NameDescriptionData) => Promise<void> | void;
};

export default function NameDescriptionForm({
  title,
  submitLabel = "Guardar",
  initialValues,
  onSubmit,
}: NameDescriptionFormProps) {
  const [name, setName] = useState(initialValues?.name ?? "");
  const [description, setDescription] = useState(
    initialValues?.description ?? "",
  );
  const [nameError, setNameError] = useState<string | undefined>();
  const [descriptionError, setDescriptionError] = useState<
    string | undefined
  >();
  const [submitting, setSubmitting] = useState(false);

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
    if (nameError) {
      setNameError(undefined);
    }
  };

  const handleSubmit = async (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
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
      setDescriptionError("La descripción no puede superar los 60 caracteres.");
      return;
    }
    setNameError(undefined);
    setDescriptionError(undefined);
    setSubmitting(true);
    try {
      await onSubmit({
        name: trimmedName,
        description: trimmedDescription,
      });
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
      </header>

      <div className="card-body">
        <FormField label="Nombre" error={nameError} required>
          <input
            className="form-control"
            value={name}
            onChange={handleChange}
            placeholder="Ej. Celulares"
          />
        </FormField>

        <FormField
          label="Descripción"
          error={descriptionError}
          className="mt-3"
        >
          <textarea
            className="form-control"
            rows={4}
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
      </div>

      <footer className="card-footer d-flex justify-content-end">
        <button type="submit" className="btn btn-primary" disabled={submitting}>
          {submitting ? "Guardando…" : submitLabel}
        </button>
      </footer>
    </form>
  );
}
