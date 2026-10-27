/** ALPM prereleases must sort BEFORE the final release under vercmp. */
export function archPackageVersion(version: string): string {
  const [withoutBuild] = version.split("+");
  const separator = withoutBuild.indexOf("-");
  if (separator < 0) return withoutBuild;
  const base = withoutBuild.slice(0, separator);
  const prerelease = withoutBuild.slice(separator + 1).replace(/[^a-zA-Z0-9]/g, "");
  return `${base}${/^[a-zA-Z]/.test(prerelease) ? "" : "pre"}${prerelease}`;
}
