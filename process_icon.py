from PIL import Image, ImageDraw

def add_corners(im, rad):
    circle = Image.new('L', (rad * 2, rad * 2), 0)
    draw = ImageDraw.Draw(circle)
    draw.ellipse((0, 0, rad * 2 - 1, rad * 2 - 1), fill=255)
    
    alpha = Image.new('L', im.size, 255)
    w, h = im.size
    
    # Four corners
    alpha.paste(circle.crop((0, 0, rad, rad)), (0, 0))
    alpha.paste(circle.crop((0, rad, rad, rad * 2)), (0, h - rad))
    alpha.paste(circle.crop((rad, 0, rad * 2, rad)), (w - rad, 0))
    alpha.paste(circle.crop((rad, rad, rad * 2, rad * 2)), (w - rad, h - rad))
    
    # Edges
    alpha.paste(255, (rad, 0, w - rad, h))
    alpha.paste(255, (0, rad, w, h - rad))
    
    # If image doesn't have alpha, add it
    if im.mode != 'RGBA':
        im = im.convert('RGBA')
        
    im.putalpha(alpha)
    return im

def main():
    try:
        input_path = 'assets/rust-finger.png'
        output_path = 'assets/rust-finger.png'
        
        # Open image
        img = Image.open(input_path)
        print(f"Original size: {img.size}, mode: {img.mode}")
        
        # Radius ~22% of width (standard squircle-ish)
        radius = int(min(img.size) * 0.22)
        
        # Process
        result = add_corners(img, radius)
        
        # Save
        result.save(output_path, 'PNG')
        print(f"Processed image saved to {output_path} with rounded corners and transparency.")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
