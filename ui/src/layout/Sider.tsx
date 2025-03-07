import { Layout, Menu } from "antd";
import { createStyles } from "antd-style";

const useStyles = createStyles(({ css }) => ({
  container: css`
    text-align: center;
    line-height: 120px;
    color: #fff;
    background-color: #1677ff;
  `,
}));

const { Sider: AntdSider } = Layout;

const Sider = () => {
  const { styles } = useStyles();
  return (
    <AntdSider
      width="25%"
      className={styles.container}
      breakpoint="lg"
      collapsedWidth="0"
      onBreakpoint={(broken) => {
        console.log(broken);
      }}
      onCollapse={(collapsed, type) => {
        console.log(collapsed, type);
      }}
    >
      <div className="demo-logo-vertical" />
      <Menu
        theme="dark"
        mode="inline"
        defaultSelectedKeys={["4"]}
        items={items}
      />
    </AntdSider>
  );
};

export default Sider;
