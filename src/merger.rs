use crate::{cell::ImageCell, core::Image, functions::paste};
use image::Pixel;
use std::{marker::Sync, ops::DerefMut};

/// Represents a point on any canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

/// The Merger trait that all mergers must implement. This trait allows the merger to paste images to a canvas.
/// # Type Parameters
/// * `P` - The pixel type of the underlying image.
pub trait Merger<P>
    where
        P: Pixel + Sync,
        <P as Pixel>::Subpixel: Sync,
{
    /// Returns a reference to the underlying canvas.
    fn get_canvas(&self) -> &Image<P, image::ImageBuffer<P, Vec<P::Subpixel>>>;

    /// Consumes the underlying merger and returns the canvas.
    fn into_canvas(self) -> Image<P, image::ImageBuffer<P, Vec<P::Subpixel>>>;

    /// Allows the merger to push an image to the canvas. This can be used in a loop to paste a large number of images without
    /// having to hold all them in memory.
    /// # Arguments
    /// * `image` - The image to push onto the canvas. Its pixel type, `P`, must match the canvas, and its `Container` must be dereferenceable to
    /// a slice of `P::Subpixel`s.
    fn push<Container>(&mut self, image: &Image<P, image::ImageBuffer<P, Container>>)
        where
            Container: DerefMut<Target=[P::Subpixel]>;
}

/// A known size merger that allows you to paste images onto a canvas. This merger is useful when you already know the size
/// of all the images being pushed onto the canvas.
/// # Type Parameters
/// * `P` - The pixel type of the underlying image.
pub struct KnownSizeMerger<P: Pixel> {
    canvas: ImageCell<P, image::ImageBuffer<P, Vec<P::Subpixel>>>,
    // The dimensions of the images being pasted (images must be a uniform size)
    num_images: u32,
    // The number of pages per row.
    last_pasted_index: i32,
    y: u32,
}

impl<P> KnownSizeMerger<P>
    where
        P: Pixel + Sync,
        <P as Pixel>::Subpixel: Sync,
{
    /// Constructs a new KnownSizeMerger.
    /// # Arguments
    /// * `image_dimensions` - The dimensions of the images being pasted (images must be a uniform size)
    /// * `images_per_row` - The number of images per row.
    /// * `total_images` - The total numbr of images to be in the final canvas.
    /// * `padding` - The padding between images, or None for no padding.
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let canvas: Image<P, image::ImageBuffer<P, Vec<P::Subpixel>>> = Image::new(
            image_width,
            image_height,
        );

        Self {
            canvas: ImageCell::new(canvas),
            num_images: 0,
            last_pasted_index: -1,
            y: 0,
        }
    }

    /// Returns the number of images that have been pasted to the canvas.
    pub fn get_num_images(&self) -> u32 {
        self.num_images
    }
}

impl<P> Merger<P> for KnownSizeMerger<P>
    where
        P: Pixel + Sync,
        <P as Pixel>::Subpixel: Sync,
{
    fn get_canvas(&self) -> &Image<P, image::ImageBuffer<P, Vec<P::Subpixel>>> {
        &self.canvas
    }

    fn into_canvas(self) -> Image<P, image::ImageBuffer<P, Vec<P::Subpixel>>> {
        self.canvas.into_inner()
    }

    fn push<Container>(&mut self, image: &Image<P, image::ImageBuffer<P, Container>>)
        where
            Container: DerefMut<Target=[P::Subpixel]>,
    {
        paste(&self.canvas, image, Point { x: 0, y: self.y });

        self.y += image.height();
        self.last_pasted_index += 1;
        self.num_images += 1;
    }
}
