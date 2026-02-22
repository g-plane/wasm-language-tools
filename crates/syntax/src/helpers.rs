#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Type that represents tree element which may be either a node or a token.
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}
impl<N, T> NodeOrToken<N, T> {
    #[inline]
    pub fn into_node(self) -> Option<N> {
        if let NodeOrToken::Node(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[inline]
    pub fn into_token(self) -> Option<T> {
        if let NodeOrToken::Token(token) = self {
            Some(token)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_node(&self) -> Option<&N> {
        if let NodeOrToken::Node(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_token(&self) -> Option<&T> {
        if let NodeOrToken::Token(token) = self {
            Some(token)
        } else {
            None
        }
    }
}
