//! Defines the [PdfPageLinkAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Link`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::link::PdfLink;
use crate::page_annotation_attachment_points::PdfPageAnnotationAttachmentPoints;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::page_objects_private::internal::PdfPageObjectsPrivate;
use crate::points::PdfPoints;

/// A single `PdfPageAnnotation` of type `PdfPageAnnotationType::Link`.
pub struct PdfPageLinkAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    attachment_points: PdfPageAnnotationAttachmentPoints<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageLinkAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageLinkAnnotation {
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            attachment_points: PdfPageAnnotationAttachmentPoints::from_pdfium(
                annotation_handle,
                bindings,
            ),
            bindings,
        }
    }

    /// Returns the [PdfLink] associated with this [PdfPageLinkAnnotation], if any.
    pub fn link(&self) -> Result<PdfLink, PdfiumError> {
        let handle = self.bindings.FPDFAnnot_GetLink(self.handle);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfLink::from_pdfium(
                handle,
                self.objects.document_handle(),
                self.bindings,
            ))
        }
    }

    /// Sets the [PdfLink] associated with this [PdfPageLinkAnnotation] to the given URI.
    pub fn set_link(&mut self, uri: &str) -> Result<(), PdfiumError> {
        if self
            .bindings()
            .is_true(self.bindings().FPDFAnnot_SetURI(self.handle, uri))
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    pub fn set_dest(&mut self, page_dest: &super::page::PdfPage, x: PdfPoints, y: PdfPoints, z: PdfPoints) -> Result<(), PdfiumError> {
        if self
            .bindings()
            .is_true(self.bindings().FPDFAnnot_SetDest(self.handle, page_dest.page_handle(), x.value, y.value, z.value))
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns a mutable collection of all the attachment points in this [PdfPageLinkAnnotation].
    #[inline]
    pub fn attachment_points_mut(&mut self) -> &mut PdfPageAnnotationAttachmentPoints<'a> {
        &mut self.attachment_points
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageLinkAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects {
        &self.objects
    }

    #[inline]
    fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a> {
        &mut self.objects
    }

    #[inline]
    fn attachment_points_impl(&self) -> &PdfPageAnnotationAttachmentPoints {
        &self.attachment_points
    }

    #[inline]
    fn attachment_points_mut_impl(&mut self) -> &mut PdfPageAnnotationAttachmentPoints<'a> {
        &mut self.attachment_points
    }
}
