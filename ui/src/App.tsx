import { Layout } from "antd";
import Sider from "@/layout/Sider";
import ThemeProvider from "@/layout/ThemeProvider";

const { Header, Content, Footer } = Layout;

const headerStyle: React.CSSProperties = {
  textAlign: "center",
  color: "#fff",
  height: 72,
  paddingInline: 48,
  lineHeight: "72px",
  backgroundColor: "#4096ff",
};

const contentStyle: React.CSSProperties = {
  textAlign: "center",
  minHeight: 120,
  lineHeight: "120px",
  color: "#fff",
  backgroundColor: "#0958d9",
};

const footerStyle: React.CSSProperties = {
  textAlign: "center",
  color: "#fff",
  backgroundColor: "#4096ff",
};

const layoutStyle = {
  overflow: "hidden",
  width: "100%",
  height: "100vh",
  maxWidth: "100%",
};

const App = () => {
  return (
    <ThemeProvider>
      <Layout style={layoutStyle}>
        <Sider />
        <Layout>
          <Header data-tauri-drag-region style={headerStyle}>
            Header
          </Header>
          <Content style={contentStyle}>Content</Content>
          <Footer style={footerStyle}>Footer</Footer>
        </Layout>
      </Layout>
    </ThemeProvider>
  );
};

export default App;
