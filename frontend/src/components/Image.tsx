import PatternInput from "./PatternInput.tsx";

function Image() {
  return (
    <PatternInput
      title="Image Pattern"
      subtitle="Upload an image to convert into a commit pattern. The image will be simplified and mapped to commit counts."
    >
      <fieldset className="fieldset ">
        <legend className="fieldset-legend">Start date</legend>
        <input type="date" className="input w-full" />
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">Pattern file</legend>
        <input type="file" className="file-input w-full" />
      </fieldset>
    </PatternInput>
  );
}

export default Image;
