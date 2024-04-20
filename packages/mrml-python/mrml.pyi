from typing import Any, Dict, Optional, Set, Union

class NoopIncludeLoaderOptions:
    """No-operation loader options class, which requires no specific configuration."""

    pass

class MemoryIncludeLoaderOptions:
    """MemoryIncludeLoaderOptions stores mappings for in-memory data using a dictionary."""
    def __init__(self, data: Dict[str, str]) -> None: ...

class LocalIncludeLoaderOptions:
    """LocalIncludeLoaderOptions configures a loader with a filesystem path."""
    def __init__(self, path: str) -> None: ...

class HttpIncludeLoaderOptionsMode:
    """HttpIncludeLoaderOptionsMode is an enumeration for HTTP loader behavior modes."""

    Allow: "HttpIncludeLoaderOptionsMode"
    Deny: "HttpIncludeLoaderOptionsMode"

class HttpIncludeLoaderOptions:
    """HttpIncludeLoaderOptions defines options for an HTTP include loader, including mode and a list of URLs."""
    def __init__(self, mode: HttpIncludeLoaderOptionsMode, list: Set[str]) -> None: ...

class ParserIncludeLoaderOptions:
    """ParserIncludeLoaderOptions is a union type that can represent any type of include loader options."""
    def __init__(
        self,
        loader: Union[
            NoopIncludeLoaderOptions,
            MemoryIncludeLoaderOptions,
            LocalIncludeLoaderOptions,
            HttpIncludeLoaderOptions,
        ],
    ) -> None: ...
    def build(self) -> Any:
        """Build method constructs the actual loader object, might return different types based on the loader configuration."""
        ...

def noop_loader() -> ParserIncludeLoaderOptions:
    """Factory function to create a no-operation loader."""
    ...

def memory_loader(data: Optional[Dict[str, str]] = None) -> ParserIncludeLoaderOptions:
    """Factory function to create a memory loader with optional data."""
    ...

def local_loader(data: Optional[str] = None) -> ParserIncludeLoaderOptions:
    """Factory function to create a local loader, converting a string path to a Path object internally."""
    ...

def http_loader(
    mode: Optional[HttpIncludeLoaderOptionsMode] = None, list: Optional[Set[str]] = None
) -> ParserIncludeLoaderOptions:
    """Factory function to create an HTTP loader with optional mode and list of URLs."""
    ...

class ParserOptions:
    """ParserOptions configures parser behavior, primarily by specifying the include loader to use."""
    def __init__(
        self, include_loader: Optional[ParserIncludeLoaderOptions] = None
    ) -> None: ...

class RenderOptions:
    """RenderOptions configures rendering behavior, including whether to disable comments and how to handle social icons and fonts."""
    def __init__(
        self,
        disable_comments: bool = False,
        social_icon_origin: Optional[str] = None,
        fonts: Optional[Dict[str, str]] = None,
    ) -> None: ...

def to_html(
    input: str,
    parser_options: Optional[ParserOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str:
    """Function to convert input a MJML string to HTML using optional parser and render configurations."""
    ...
