import { css } from "@linaria/core";
import { heading1Text } from "~/style/commonStyle";

const Page1 = () => {
  return <div class={Foo}>Page 1</div>;
};

const Foo = css`
  ${heading1Text}
`;

export default Page1;
