import { useContext, useEffect } from "react";

import { FormContext } from "../context.tsx";
import { useEmailInput, useGithubBranch, useGithubName, useTimezoneInput } from "../hooks.ts";
import InputText from "./InputText.tsx";

function RepoDetail() {
  const { updateFormContext } = useContext(FormContext);
  const [name, nameErr, onNameChange, nameRef] = useGithubName();
  const [username, usernameErr, onUsernameChange, usernameRef] = useGithubName();
  const [email, emailError, onEmailChange, emailRef] = useEmailInput();
  const [branch, branchErr, onBranchChange, branchRef] = useGithubBranch();
  const [timezone, timezoneErr, onTimezoneChange, timezoneRef] = useTimezoneInput();

  useEffect(() => {
    updateFormContext({ name, username, email, branch, timezone });
  }, [name, username, email, branch, timezone, updateFormContext]);

  useEffect(() => {
    const inputErrors: Record<string, boolean> = {
      nameErr: !!nameErr,
      usernameErr: !!usernameErr,
      emailError: !!emailError,
      branchErr: !!branchErr,
      timezoneErr: !!timezoneErr,
    };
    updateFormContext({ inputErrors });
  }, [nameErr, usernameErr, emailError, branchErr, timezoneErr, updateFormContext]);

  return (
    <>
      <div className="text-xl font-bold px-1 py-4">Enter repo detail</div>
      <div className="card bg-neutral text-neutral-content shadow-sm">
        <div className="card-body">
          <div className="my-4 grid gap-x-4 gap-y-1 grid-cols-1 sm:grid-cols-2">
            <InputText
              legend="Repo name"
              placeholder="Repo name"
              onChange={onNameChange}
              inputRef={nameRef}
              error={nameErr}
            />
            <InputText
              legend="Username"
              placeholder="Username"
              onChange={onUsernameChange}
              inputRef={usernameRef}
              error={usernameErr}
            />
            <InputText
              legend="Email"
              placeholder="Email must be linked to your GitHub account"
              onChange={onEmailChange}
              inputRef={emailRef}
              type="email"
              error={emailError}
            />
            <InputText
              legend="Branch"
              placeholder="Branch"
              onChange={onBranchChange}
              inputRef={branchRef}
              error={branchErr}
            />
            <InputText
              legend="Timezone"
              placeholder="+0700"
              onChange={onTimezoneChange}
              inputRef={timezoneRef}
              error={timezoneErr}
            />
          </div>
        </div>
      </div>
    </>
  );
}

export default RepoDetail;
