import { useCommitCountInput, useDateInput, useFontInput, useTextInput } from "../hooks.ts";
import InputCommitCount from "./InputCommitCount.tsx";
import InputDate from "./InputDate.tsx";
import InputFont from "./InputFont.tsx";
import InputText from "./InputText.tsx";
import PatternInput from "./PatternInput.tsx";

function Text() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput();
  const [commitCount, onCommitCountChange] = useCommitCountInput();
  const [font, onFontChange] = useFontInput();
  const [text, onTextChange, textInputRef] = useTextInput(250);

  return (
    <PatternInput
      title="Text Pattern"
      subtitle="Display text on your commit graph. The text will be converted into a commit pattern"
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputCommitCount value={commitCount} onChange={onCommitCountChange} />
      <InputFont value={font} onChange={onFontChange} />
      <InputText legend="Text" placeholder="Keep it short" onChange={onTextChange} inputRef={textInputRef} />
    </PatternInput>
  );
}

export default Text;
