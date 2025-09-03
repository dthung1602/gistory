import PatternInput from "./PatternInput.tsx";

function TextFilePattern() {
  return (
    <PatternInput
      title="Text File Pattern"
      subtitle="Upload a text file containing a pattern to draw on your commit graph.
                Each line should represent a week, and each character a day. Use 0-4 to indicate commit counts"
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

export default TextFilePattern;
