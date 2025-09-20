import { useContext, useRef, useState } from "react";

import api from "../api.tsx";
import GenerateIcon from "../asset/auto-generate-svgrepo-com.svg?react";
import DownloadIcon from "../asset/download-svgrepo-com.svg?react";
import { FormContext, ToastContext } from "../context.tsx";
import type { CreateRepoData, Repo } from "../types.ts";

function GenerateButton() {
  const { addToast } = useContext(ToastContext);
  const { inputErrors, name, username, branch, email, startDate, timezone, data } = useContext(FormContext);
  const [downloadLink, setDownloadLink] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const disabled = Object.values(inputErrors).some(Boolean);

  const handleClick = async () => {
    if (disabled || loading) return;

    const textFields = [name, username, branch, email, startDate, timezone];
    const emptyField = data.length == 0 || textFields.includes("") || textFields.includes(null);

    if (emptyField) {
      addToast({ type: "success", content: "Please fill in all fields" });
      return;
    }

    setLoading(true);
    const [resPromise] = api.createRepo({
      name,
      username,
      branch,
      email,
      startDate,
      timezone,
      data,
    } as CreateRepoData);

    function sleep(ms: number) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function checkRepoStatus(repo: Repo) {
      while (repo.status === "InProgress" || repo.status === "New") {
        await sleep(5000);
        const res = await api.getRepo(repo.uuid)[0];
        repo = (await res.json()) as Repo;
      }
      return repo;
    }

    resPromise
      .then(async res => {
        if (!res.ok) {
          return;
        }
        let repo = (await res.json()) as Repo;
        repo = await checkRepoStatus(repo);
        if (repo.status === "Error") {
          addToast({
            key: Math.random(),
            type: "error",
            content: "Error creating repo. Try again later.",
          });
          setError("Error creating repo. Try again later.");
          setDownloadLink("");
        } else {
          addToast({
            key: Math.random(),
            type: "success",
            content: "Start downloading...",
          });
          const link = api.genDownloadUrl(repo.uuid, repo.name);
          api.download(link);
          setDownloadLink(link);
          setError("");
        }
      })
      .catch(api.errHandler(addToast, setError))
      .finally(() => {
        setLoading(false);
      });
  };

  let buttonClass = "btn btn-lg";
  if (disabled) {
    buttonClass += " btn-disabled";
  }
  if (loading) {
    buttonClass += " gradient-animated text-secondary-content";
  } else {
    buttonClass += " btn-secondary";
  }

  return (
    <div className="flex flex-col justify-center mt-8">
      <button onClick={handleClick} className={buttonClass}>
        {loading ? <span className="loading loading-spinner loading-sm" /> : <GenerateIcon className="w-6 h-6" />}
        Generate repo
      </button>
      <span className="text-error my-4">{error}</span>
      {downloadLink && [
        <button
          className={`btn btn-lg btn-dash btn-primary py-12 text-lg ${loading ? "btn-disabled" : ""}`}
          onClick={() => api.download(downloadLink)}
        >
          <DownloadIcon className="w-8 h-8 primary-sgv" />
          <p>Download repo</p>
        </button>,
        <p className="text-success text-center mt-8">The downloaded repo is empty, with only the .git directory</p>,
      ]}
    </div>
  );
}

export default GenerateButton;
