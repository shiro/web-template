import { css } from "@linaria/core";
import EchoPlugin from "~/EchoPlugin";
import { isNative } from "~/platform";
import { heading1Text } from "~/style/commonStyle";

const Main = () => {
  let text = $signal("not clicked");

  const handle = async () => {
    text = "clicked!";
    if (isNative) {
      const { value } = await EchoPlugin.echo({ value: "Hello plugin!" });
      console.log("ping", value);
    }
  };

  return (
    <div class={Foo}>
      hello world {process.env.NODE_ENV}
      <br />
      <button id="ping" onClick={handle}>
        click me
      </button>
      <br />
      {text}
    </div>
  );
};

const Foo = css`
  ${heading1Text}
`;

export default Main;
