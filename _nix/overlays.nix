{self, ...}: {
  default = final: prev: {
    rudof = {
      rudof = self.packages.${final.system}.rudof;
      rudof-generate = self.packages.${final.system}.rudof-generate;
    };
  };
}
