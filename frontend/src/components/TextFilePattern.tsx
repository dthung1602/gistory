import { useDateInput, useFileInput } from "../hooks.ts";
import InputDate from "./InputDate.tsx";
import InputFile from "./InputFile.tsx";
import PatternInput from "./PatternInput.tsx";

function TextFilePattern() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput({ mustBeSunday: true });
  const [file, onChange] = useFileInput();

  return (
    <PatternInput
      title="Text File Pattern"
      subtitle={
        <span>
          Upload a text file containing a pattern to draw on your commit graph. <br />
          The text file must have fewer than 7 rows. Use 0-4 to indicate commit counts
        </span>
      }
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputFile legend="Pattern file" accept="text/plain" onChange={onChange} />
    </PatternInput>
  );
}

export default TextFilePattern;
