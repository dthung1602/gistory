import * as React from "react";

type Prop = {
  title: string;
  subtitle: string;
  children: React.ReactNode;
};

function PatternInput({ title, subtitle, children }: Prop) {
  return (
    <div className="card bg-neutral text-neutral-content shadow-sm">
      <div className="card-body">
        <h2 className="card-title">{title}</h2>
        <p>{subtitle}</p>

        <div className="my-4 grid gap-4 grid-cols-1 md:grid-cols-2">
          {children}
        </div>

        <div className="card-actions">
          <button className="btn btn-secondary">Generate</button>
        </div>
      </div>
    </div>
  );
}

export default PatternInput;
