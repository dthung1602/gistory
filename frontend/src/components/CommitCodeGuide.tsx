import { useContext } from "react";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
import DownArrowSvg from "../asset/curve-arrow-down-svgrepo-com.svg?react";
import screenshot from "../asset/screenshot.png";
import { FormContext } from "../context.tsx";

function CommitCodeGuide() {
  let { branch, username, name: repoName } = useContext(FormContext);

  branch = branch || "main";
  username = username || "your-username";
  repoName = repoName || "repo-name";

  return (
    <>
      <div className="text-xl font-bold px-1 py-8">Finally, add repo to Github</div>
      <ul className="steps steps-vertical overflow-visible">
        <li data-content="★" className="step step-neutral">
          <span>
            Create repo<code>{repoName}</code>on Github
          </span>
        </li>
        <li data-content="+" className="step step-neutral">
          <span>
            Add remote
            <code>
              git remote add origin git@github.com:{username}/{repoName}.git
            </code>
          </span>
        </li>
        <li data-content="↑" className="step step-neutral">
          <span>
            Push to remote
            <code>git push -u origin {branch}</code>
          </span>
        </li>
        <li data-content="●" className="step step-neutral">
          Wait a few minutes
        </li>
        <li data-content="✓" className="step step-neutral">
          <span>
            Go to your homepage on Github
            <DownArrowSvg className="w-16 h-16 translate-y-6 translate-x-2 hidden sm:inline" />
          </span>
        </li>
      </ul>

      <div className="justify-center px-4 py-8 hidden sm:flex">
        <img className="border border-success rounded-lg w-fit" src={screenshot} />
      </div>

      <div className="text-xl font-bold px-1 pt-16 pb-4">How this works?</div>
      <p className="px-2 pb-4">
        Github updates your commit graph retroactively using the timestamp of the commit itself, not when the commit
        pushed to Github
      </p>
      <p className="px-2 pb-4">Gistory creates a bunch of empty commits to simulate the commit history of the repo</p>

      <div className="text-xl font-bold px-1 py-8">Run locally</div>
      <p className="px-2 pb-4">
        Make sure you have{" "}
        <a className="link" target="_blank" href="https://doc.rust-lang.org/cargo/getting-started/installation.html">
          <span className="text-primary/70">rust</span> & cargo installed
        </a>
        , then run
        <code>cargo install gistory</code>
      </p>
      <p className="px-2 pb-4">
        The CLI is available at <code>~/.cargo/bin/gistory</code>
      </p>
      <p className="px-2 pb-4">
        To use <code>gistory</code> crate in your rust code, see{" "}
        <a className="link" target="_blank" href="https://github.com/dthung1602/gistory?tab=readme-ov-file#library">
          this guide
        </a>
      </p>
    </>
  );
}

export default CommitCodeGuide;
