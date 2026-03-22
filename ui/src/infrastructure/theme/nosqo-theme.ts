import { createTheme } from "@mantine/core";

export const nosqoTheme = createTheme({
  primaryColor: "teal",
  defaultRadius: "md",
  fontFamily: '"IBM Plex Sans", "Segoe UI", sans-serif',
  spacing: {
    xs: "0.4rem",
    sm: "0.65rem",
    md: "0.9rem",
    lg: "1.15rem",
    xl: "1.4rem",
  },
  headings: {
    fontFamily: '"IBM Plex Sans", "Segoe UI", sans-serif',
    fontWeight: "600",
  },
  components: {
    Button: {
      defaultProps: {
        size: "sm",
      },
    },
    TextInput: {
      defaultProps: {
        size: "sm",
      },
    },
    Textarea: {
      defaultProps: {
        size: "sm",
      },
    },
    Badge: {
      defaultProps: {
        size: "sm",
      },
    },
    Paper: {
      defaultProps: {
        radius: "lg",
      },
    },
  },
});
