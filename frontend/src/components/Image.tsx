import { useDateInput, useFileInput } from "../hooks.ts";
import InputDate from "./InputDate.tsx";
import InputFile from "./InputFile.tsx";
import PatternInput from "./PatternInput.tsx";

function Image() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput({ mustBeSunday: true });
  const [file, onChange] = useFileInput();

  return (
    <PatternInput
      title="Image Pattern"
      subtitle="Upload an image to convert into a commit pattern. The image will be simplified and mapped to commit counts."
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputFile legend="Image" accept="image/*" onChange={onChange} />
    </PatternInput>
  );
}

export default Image;
