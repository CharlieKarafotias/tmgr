import subprocess
import sys

def check_git_installed():
    result = subprocess.run(["git", "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return result.returncode == 0

def check_gh_cli_installed():
    result = subprocess.run(["gh", "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return result.returncode == 0

def is_merge_commit():
    result = subprocess.run(
        ["git", "rev-parse", "-q", "--verify", "MERGE_HEAD"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    return result.returncode == 0

def get_current_branch():
    result = subprocess.run(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        stdout=subprocess.PIPE,
        text=True
    )
    return result.stdout.strip()

def get_gh_token():
    result = subprocess.run(
        ["gh", "auth", "token"],
        stdout=subprocess.PIPE,
        text=True
    )
    return result.stdout.strip()

def get_binary_version():
    result = subprocess.run(
        ["cargo", "pkgid"],
        stdout=subprocess.PIPE,
        text=True
    )
    return result.stdout.strip().split('#')[1]

def check_tag_exists(tag):
    result = subprocess.run(
        ["git", "tag", "-l", tag],
        stdout=subprocess.PIPE,
        text=True
    )
    return tag in result.stdout.strip()

def create_binary_release():
    result = subprocess.run(
        ["cargo", "build", "--release"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    return result.returncode == 0

def create_gh_release(version):
    result = subprocess.run(
        ['gh', 'release create', version, './target/release/tmgr', '--latest', '--generate-notes'],
        stdout=subprocess.PIPE,
        text=True
    )
    return result.stdout.strip()

if __name__ == "__main__":
    if not check_git_installed():
        print("Git is not installed. Please install Git.")
        sys.exit(1)

    if not check_gh_cli_installed():
        print("GitHub CLI (gh) is not installed. Please install GitHub CLI.")
        sys.exit(1)

    if not get_gh_token():
        print("GitHub token not found. Please set up GitHub token.")
        sys.exit(1)

    if get_current_branch() != "main":
        print("Not on the main branch. Skipping release.")
        sys.exit(0)

    if not is_merge_commit():
        print("Not a merge commit. Skipping release.")
        sys.exit(0)

    binary_version = f'v{get_binary_version()}'

    if check_tag_exists(binary_version):
        print(f"Tag {binary_version} already exist. Skipping release.")
        sys.exit(0)

    create_binary_release()
    create_gh_release(binary_version)