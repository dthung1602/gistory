import * as React from "react";

import Preview from "./Preview.tsx";

function RepoDetail() {
  return (
    <div className="w-full flex justify-center p-4">
      <div className="max-w-[900px]">
        <div className="text-xl font-bold px-1 py-4">Enter repo detail</div>
        <div className="card bg-neutral text-neutral-content shadow-sm">
          <div className="card-body">
            <h2 className="card-title">title</h2>
            <p>subtitle</p>
            <div className="my-4 grid gap-x-4 gap-y-1 grid-cols-1 md:grid-cols-2">
              <span>lkjlk;j</span>
              <span>lkjlk;j</span>
              <span>lkjlk;j</span>
            </div>
            <div className="card-actions justify-center">
              <button className={`btn btn-secondary`}>Generate Repo</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default RepoDetail;
