import { css } from "@linaria/core";
import { heading1Text } from "~/style/commonStyle";

const Page2 = () => {
  return <div class={Foo}>Page 2</div>;
};

const Foo = css`
  ${heading1Text}
`;

export default Page2;
