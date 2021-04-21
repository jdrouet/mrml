export interface MjmlAbstract<Type, Attributes, Children> {
  type: Type;
  attributes?: Attributes;
  children?: Children;
}

export type AnyAttributes = { [key: string]: string };

// Comment
export interface Comment extends MjmlAbstract<'comment', undefined, string> {}

// mj-attributes > mj-all
export interface MjAttributesAll
  extends MjmlAbstract<'mj-all', AnyAttributes, undefined> {}

// mj-attributes > mj-class
export type MjAttributesClassAttributes = { name: string } & AnyAttributes;
export interface MjAttributesClass
  extends MjmlAbstract<'mj-class', MjAttributesClassAttributes, undefined> {}

// mj-attributes > *
export interface MjAttributesElement
  extends MjmlAbstract<string, AnyAttributes, undefined> {}

// mj-attributes
export type MjAttributesChild =
  | MjAttributesAll
  | MjAttributesClass
  | MjAttributesElement;
export interface MjAttributes
  extends MjmlAbstract<'mj-attributes', undefined, Array<MjAttributesChild>> {}

// mj-breakpoint
export interface MjBreakpoint
  extends MjmlAbstract<'mj-breakpoint', { width: string }, undefined> {}

// mj-font
export interface MjFont
  extends MjmlAbstract<'mj-font', { name: string; href: string }, undefined> {}

// mj-preview
export interface MjPreview
  extends MjmlAbstract<'mj-preview', undefined, string> {}

// mj-style
export type MjStyleAttributes = { inline?: 'inline' };
export interface MjStyle
  extends MjmlAbstract<'mj-style', MjStyleAttributes, string> {}

// mj-title
export interface MjTitle extends MjmlAbstract<'mj-title', undefined, string> {}

// mj-head
export type MjHeadChild =
  | Comment
  | MjAttributes
  | MjBreakpoint
  | MjFont
  | MjPreview
  | MjTitle;
export interface MjHead
  extends MjmlAbstract<'mj-head', undefined, Array<string>> {}

// node
export interface MjNode<Child>
  extends MjmlAbstract<string, AnyAttributes, Array<Child>> {}
type MjRawChild = Comment | MjNode<MjRawChild> | string;

// mj-accordion > mj-accordion-element > mj-accordion-text
export type MjAccordionTextAttributes = {
  'background-color': string;
  'color': string;
  'css-class': string;
  'font-family': string;
  'font-size': string;
  'font-weight': string;
  'letter-spacing': string;
  'line-height': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
};
export interface MjAccordionText
  extends MjmlAbstract<
    'mj-accordion-text',
    Partial<MjAccordionTextAttributes>,
    Array<MjRawChild>
  > {}

// mj-accordion > mj-accordion-element > mj-accordion-title
export type MjAccordionTitleAttributes = {
  'background-color': string;
  'color': string;
  'css-class': string;
  'font-family': string;
  'font-size': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
};
export interface MjAccordionTitle
  extends MjmlAbstract<
    'mj-accordion-title',
    Partial<MjAccordionTitleAttributes>,
    Array<string>
  > {}

// mj-accordion > mj-accordion-element
export type MjAccordionElementAttributes = {
  'background-color': string;
  'border': string;
  'css-class': string;
  'font-family': string;
  'icon-align': string;
  'icon-height': string;
  'icon-position': string;
  'icon-unwrapped-alt': string;
  'icon-unwrapped-url': string;
  'icon-width': string;
  'icon-wrapped-alt': string;
  'icon-wrapped-url': string;
};
type MjAccordionElementChild = Comment | MjAccordionText | MjAccordionTitle;
export interface MjAccordionElement
  extends MjmlAbstract<
    'mj-accordion-element',
    Partial<MjAccordionElementAttributes>,
    Array<MjAccordionElementChild>
  > {}

// mj-accordion
export type MjAccordionAttributes = {
  'border': string;
  'container-background-color': string;
  'css-class': string;
  'font-family': string;
  'icon-align': string;
  'icon-height': string;
  'icon-position': string;
  'icon-unwrapped-alt': string;
  'icon-unwrapped-url': string;
  'icon-width': string;
  'icon-wrapped-alt': string;
  'icon-wrapped-url': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
};
export type MjAccordionChild = MjAccordionElement;
export interface MjAccordion
  extends MjmlAbstract<
    'mj-accordion',
    Partial<MjAccordionAttributes>,
    Array<MjAccordionChild>
  > {}

// mj-button
export type MjButtonAttributes = {
  'align': string;
  'background-color': string;
  'border': string;
  'border-top': string;
  'border-right': string;
  'border-bottom': string;
  'border-left': string;
  'border-radius': string;
  'color': string;
  'container-background-color': string;
  'css-class': string;
  'font-family': string;
  'font-size': string;
  'font-style': string;
  'font-weight': string;
  'height': string;
  'href': string;
  'inner-padding': string;
  'letter-spacing': string;
  'line-height': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'rel': string;
  'target': string;
  'text-align': string;
  'text-decoration': string;
  'text-transform': string;
  'title': string;
  'vertical-align': string;
  'width': string;
};
export interface MjButton
  extends MjmlAbstract<
    'mj-button',
    Partial<MjButtonAttributes>,
    Array<MjBodyChild>
  > {}

// mj-carousel > mj-carousel-image
export type MjCarouselImageAttributes = {
  'alt': string;
  'css-class': string;
  'href': string;
  'rel': string;
  'src': string;
  'target': string;
  'thumbnails-src': string;
  'title': string;
};
export interface MjCarouselImage
  extends MjmlAbstract<
    'mj-carousel-image',
    Partial<MjCarouselImageAttributes>,
    undefined
  > {}

// mj-carousel
export type MjCarouselAttributes = {
  'align': string;
  'background-color': string;
  'border-radius': string;
  'css-class': string;
  'icon-width': string;
  'left-icon': string;
  'right-icon': string;
  'tb-border': string;
  'tb-border-radius': string;
  'tb-hover-border-color': string;
  'tb-selected-border-color': string;
  'tb-width': string;
  'thumbnails': string;
};
export interface MjCarousel
  extends MjmlAbstract<
    'mj-carousel',
    Partial<MjCarouselAttributes>,
    Array<Comment | MjCarouselImage>
  > {}

// mj-column
export type MjColumnAttributes = {
  'background-color': string;
  'border': string;
  'border-top': string;
  'border-right': string;
  'border-bottom': string;
  'border-left': string;
  'border-radius': string;
  'css-class': string;
  'inner-background-color': string;
  'inner-border': string;
  'inner-border-top': string;
  'inner-border-right': string;
  'inner-border-bottom': string;
  'inner-border-left': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'vertical-align': string;
  'width': string;
};
export interface MjColumn
  extends MjmlAbstract<
    'mj-column',
    Partial<MjColumnAttributes>,
    Array<MjBodyChild>
  > {}

// mj-divider
export type MjDividerAttributes = {
  'align': string;
  'border-color': string;
  'border-style': string;
  'border-width': string;
  'container-background-color': string;
  'css-class': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'width': string;
};
export interface MjDivider
  extends MjmlAbstract<'mj-divider', Partial<MjDividerAttributes>, undefined> {}

// mj-group
export type MjGroupAttributes = {
  'width': string;
  'vertical-align': string;
  'background-color': string;
  'direction': string;
  'css-class': string;
};
export interface MjGroup
  extends MjmlAbstract<
    'mj-group',
    Partial<MjGroupAttributes>,
    Array<MjBodyChild>
  > {}

// mj-hero
export type MjHeroAttributes = {
  'background-color': string;
  'background-height': string;
  'background-position': string;
  'background-url': string;
  'background-width': string;
  'border-radius': string;
  'height': string;
  'mode': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'vertical-align': string;
  'width': string;
};
export interface MjHero
  extends MjmlAbstract<
    'mj-hero',
    Partial<MjHeroAttributes>,
    Array<MjBodyChild>
  > {}

// mj-image
export type MjImageAttributes = {
  'align': string;
  'alt': string;
  'border': string;
  'border-top': string;
  'border-right': string;
  'border-bottom': string;
  'border-left': string;
  'border-radius': string;
  'container-background-color': string;
  'css-class': string;
  'fluid-on-mobile': string;
  'height': string;
  'href': string;
  'name': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'rel': string;
  'sizes': string;
  'src': string;
  'srcset': string;
  'target': string;
  'title': string;
  'usemap': string;
  'width': string;
};
export interface MjImage
  extends MjmlAbstract<'mj-image', Partial<MjImageAttributes>, undefined> {}

// mj-navbar > mj-navbar-link
export type MjNavbarLinkAttributes = {
  'color': string;
  'css-class': string;
  'font-family': string;
  'font-size': string;
  'font-style': string;
  'font-weight': string;
  'href': string;
  'letter-spacing': string;
  'line-height': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'rel': string;
  'target': string;
  'text-decoration': string;
  'text-transform': string;
};
export interface MjNavbarLink
  extends MjmlAbstract<
    'mj-navbar-link',
    Partial<MjNavbarLinkAttributes>,
    Array<Comment | string>
  > {}

// mj-navbar
export type MjNavbarAttributes = {
  'align': string;
  'base-url': string;
  'css-class': string;
  'hamburger': string;
  'ico-align': string;
  'ico-close': string;
  'ico-color': string;
  'ico-font-family': string;
  'ico-font-size': string;
  'ico-line-height': string;
  'ico-open': string;
  'ico-padding': string;
  'ico-padding-top': string;
  'ico-padding-right': string;
  'ico-padding-bottom': string;
  'ico-padding-left': string;
  'ico-text-decoration': string;
  'ico-text-transform': string;
};
export interface MjNavbar
  extends MjmlAbstract<
    'mj-navbar',
    MjNavbarAttributes,
    Array<Comment | MjNavbarLink>
  > {}

// mj-raw
export interface MjRaw
  extends MjmlAbstract<'mj-raw', undefined, Array<MjRawChild>> {}

// mj-section
export type MjSectionAttributes = {
  'background-color': string;
  'background-position': string;
  'background-position-x': string;
  'background-position-y': string;
  'background-repeat': string;
  'background-size': string;
  'background-url': string;
  'border': string;
  'border-top': string;
  'border-right': string;
  'border-bottom': string;
  'border-left': string;
  'border-radius': string;
  'css-class': string;
  'direction': string;
  'full-width': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'text-align': string;
};
export interface MjSection
  extends MjmlAbstract<
    'mj-section',
    Partial<MjSectionAttributes>,
    Array<MjBodyChild>
  > {}

// mj-social > mj-social-element
export type MjSocialElementAttributes = {
  'align': string;
  'alt': string;
  'background-color': string;
  'border-radius': string;
  'color': string;
  'css-class': string;
  'font-family': string;
  'font-size': string;
  'font-style': string;
  'font-weight': string;
  'href': string;
  'icon-height': string;
  'icon-size': string;
  'line-height': string;
  'name': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'text-padding': string;
  'sizes': string;
  'src': string;
  'srcset': string;
  'rel': string;
  'target': string;
  'title': string;
  'text-decoration': string;
  'vertical-align': string;
};
export interface MjSocialElement
  extends MjmlAbstract<
    'mj-social-element',
    Partial<MjSocialElementAttributes>,
    Array<Comment | string>
  > {}

// mj-social
export type MjSocialAttributes = {
  'align': string;
  'border-radius': string;
  'color': string;
  'css-class': string;
  'container-background-color': string;
  'font-family': string;
  'font-size': string;
  'font-style': string;
  'font-weight': string;
  'icon-height': string;
  'icon-size': string;
  'inner-padding': string;
  'line-height': string;
  'mode': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'icon-padding': string;
  'text-padding': string;
  'text-decoration': string;
};
export interface MjSocial
  extends MjmlAbstract<
    'mj-social',
    Partial<MjSocialAttributes>,
    Array<Comment | MjSocialElement>
  > {}

// mj-spacer
export type MjSpacerAttributes = {
  'container-background-color': string;
  'css-class': string;
  'height': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
};
export interface MjSpacer
  extends MjmlAbstract<'mj-spacer', Partial<MjSpacerAttributes>, undefined> {}

// mj-text
export type MjTextAttributes = {
  'color': string;
  'font-family': string;
  'font-size': string;
  'font-style': string;
  'font-weight': string;
  'line-height': string;
  'letter-spacing': string;
  'height': string;
  'text-decoration': string;
  'text-transform': string;
  'align': string;
  'container-background-color': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'css-class': string;
};
export interface MjText
  extends MjmlAbstract<
    'mj-text',
    Partial<MjTextAttributes>,
    Array<MjBodyChild>
  > {}

// mj-wrapper
export type MjWrapperAttributes = {
  'background-color': string;
  'background-position': string;
  'background-position-x': string;
  'background-position-y': string;
  'background-repeat': string;
  'background-size': string;
  'background-url': string;
  'border': string;
  'border-top': string;
  'border-right': string;
  'border-bottom': string;
  'border-left': string;
  'border-radius': string;
  'css-class': string;
  'full-width': string;
  'padding': string;
  'padding-top': string;
  'padding-right': string;
  'padding-bottom': string;
  'padding-left': string;
  'text-align': string;
};
export interface MjWrapper
  extends MjmlAbstract<
    'mj-wrapper',
    Partial<MjWrapperAttributes>,
    Array<MjBodyChild>
  > {}

// mj-body
export type MjBodyChild =
  | Comment
  | MjAccordion
  | MjButton
  | MjCarousel
  | MjColumn
  | MjDivider
  | MjGroup
  | MjHero
  | MjImage
  | MjNavbar
  | MjRaw
  | MjSection
  | MjSocial
  | MjSpacer
  | MjWrapper
  | MjNode<MjBodyChild>
  | string;
export type MjBodyAttributes = {
  'background-color': string;
  'css-class': string;
  'width': string;
};
export interface MjBody
  extends MjmlAbstract<
    'mj-body',
    Partial<MjBodyAttributes>,
    Array<MjBodyChild>
  > {}

// mjml
export type MjmlAttributes = { lang: string };
export interface Mjml
  extends MjmlAbstract<
    'mjml',
    Partial<MjmlAttributes>,
    Array<MjHead | MjBody>
  > {}

// all elements
export type MjmlElement = Mjml | MjHead | MjHeadChild | MjBody | MjBodyChild;
