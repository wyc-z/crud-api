import type { ReactNode } from "react";

type FormFieldProps = {
  label: string;
  error?: string;
  required?: boolean;
  className?: string;
  small?: boolean;
  children: ReactNode;
};

export default function FormField({
  label,
  error,
  required,
  className = "mb-3",
  small = false,
  children,
}: FormFieldProps) {
  return (
    <div className={className}>
      <label className={small ? "form-label small mb-1" : "form-label"}>
        {label}
        {required ? <span className="text-danger"> *</span> : null}
      </label>
      {children}
      {error ? (
        <div className="invalid-feedback d-block" role="alert">
          {error}
        </div>
      ) : null}
    </div>
  );
}