use std::marker::PhantomData;

use crate::graphics::{Line, Triangle, Vertex};

/// A model made of a line list.
pub struct LineList<V: Vertex> {
    pub primitives: Vec<Line<V>>,
}

impl<V: Vertex> LineList<V> {
    pub fn from_builder<B>(mut builder: B) -> Self
    where
        B: ModelBuilder<Line<V>, V>,
    {
        Self {
            primitives: builder.build_primitives(),
        }
    }
}

/// A model made of a line list with indexed vertices.
pub struct IndexedLineList<V: Vertex> {
    pub vertices: Vec<V>,
    pub primitives: Vec<Line<usize>>,
}

impl<V: Vertex> IndexedLineList<V> {
    pub fn from_builder<B>(mut builder: B) -> Self
    where
        B: ModelBuilder<Line<usize>, V>,
    {
        Self {
            primitives: builder.build_primitives(),
            vertices: builder.build_vertices(),
        }
    }
}

/// A model made of a triangle list.
pub struct TriangleList<V: Vertex> {
    pub primitives: Vec<Triangle<V>>,
}

impl<V: Vertex> TriangleList<V> {
    pub fn from_builder<B>(mut builder: B) -> Self
    where
        B: ModelBuilder<Triangle<V>, V>,
    {
        Self {
            primitives: builder.build_primitives(),
        }
    }
}

/// A model made of a triangle list with indexed vertices.
pub struct IndexedTriangleList<V: Vertex> {
    pub vertices: Vec<V>,
    pub primitives: Vec<Triangle<usize>>,
}

impl<V: Vertex> IndexedTriangleList<V> {
    pub fn from_builder<B>(mut builder: B) -> Self
    where
        B: ModelBuilder<Triangle<usize>, V>,
    {
        Self {
            primitives: builder.build_primitives(),
            vertices: builder.build_vertices(),
        }
    }
}

/// Trait for procedural model generation.
pub trait ModelBuilder<P, V> {
    fn build_primitives(&mut self) -> Vec<P>;

    fn build_vertices(&mut self) -> Vec<V>;

    fn transform<VO, T>(&mut self, transformer: T) -> ModelAdaptor<P, V, Self, VO, T>
    where
        T: FnMut(V) -> VO, 
        Self: Sized,
    {
        ModelAdaptor::new(self, transformer)
    }
}

pub struct ModelAdaptor<'a, P, VI, B, VO, T>
where
    B: ModelBuilder<P, VI>,
    T: FnMut(VI) -> VO,
{
    builder: &'a mut B,
    transformer: T,
    primitive: PhantomData<P>,
    vertex_input: PhantomData<VI>,
}

impl<'a, P, VI, B, VO, T> ModelAdaptor<'a, P, VI, B, VO, T>
where
    B: ModelBuilder<P, VI>,
    T: FnMut(VI) -> VO,
{
    pub fn new(builder: &'a mut B, transformer: T) -> Self {
        Self {
            builder,
            transformer,
            primitive: PhantomData::default(),
            vertex_input: PhantomData::default(),
        }
    }
}

impl<'a, P, VI, B, VO, T> ModelBuilder<P, VO> for ModelAdaptor<'a, P, VI, B, VO, T>
where
    B: ModelBuilder<P, VI>,
    T: FnMut(VI) -> VO,
{
    fn build_primitives(&mut self) -> Vec<P> {
        self.builder.build_primitives()
    }

    fn build_vertices(&mut self) -> Vec<VO> {
        self.builder
            .build_vertices()
            .into_iter()
            .map(&mut self.transformer)
            .collect()
    }
}
