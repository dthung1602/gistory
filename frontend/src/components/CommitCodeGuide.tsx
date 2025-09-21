import { useContext } from "react";

import { FormContext } from "../context.tsx";

function CommitCodeGuide() {
  let { branch, username, name: repoName } = useContext(FormContext);

  branch = branch || "main";
  username = username || "your-username";
  repoName = repoName || "repo-name";

  return (
    <>
      <div className="text-xl font-bold px-1 py-8">Finally, add repo to Github</div>
      <ul className="steps steps-vertical">
        <li data-content="★" className="step step-neutral">
          <span>
            Create repo<code className="border-neutral-content border py-1 px-2 rounded-sm mx-1">{repoName}</code>on
            Github
          </span>
        </li>
        <li data-content="+" className="step step-neutral">
          <span>
            Add remote
            <code className="border-neutral-content border py-1 px-2 rounded-sm mx-1">
              git remote add origin git@github.com:{username}/{repoName}.git
            </code>
          </span>
        </li>
        <li data-content="↑" className="step step-neutral">
          <span>
            Push to remote
            <code className="border-neutral-content border py-1 px-2 rounded-sm mx-1">git push -u origin {branch}</code>
          </span>
        </li>
        <li data-content="●" className="step step-neutral">
          Wait a few minutes
        </li>
        <li data-content="✓" className="step step-neutral">
          Go to your homepage on Github
        </li>
      </ul>
      <div className="text-xl font-bold px-1 pt-8 pb-4">How this works?</div>
      <p className="px-2 pb-4">
        Github updates your commit graph retroactively using the timestamp of the commit itself, not when the commit
        pushed to Github
      </p>
      <p className="px-2 pb-4">Gistory creates a bunch of empty commits to simulate the commit history of the repo</p>
    </>
  );
}

export default CommitCodeGuide;
