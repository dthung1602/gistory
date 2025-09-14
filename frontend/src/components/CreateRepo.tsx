import { FormContextProvider } from "../context.tsx";
import GenerateButton from "./GenerateButton.tsx";
import RepoDetail from "./RepoDetail.tsx";
import SelectPattern from "./SelectPattern.tsx";

function CreateRepo() {
  return (
    <FormContextProvider>
      <div className="w-full flex justify-center px-4">
        <div>
          <SelectPattern />
          <RepoDetail />
          <GenerateButton />
        </div>
      </div>
    </FormContextProvider>
  );
}

export default CreateRepo;
