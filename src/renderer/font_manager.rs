use std::cmp::max;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use crate::renderer::Texture;

pub struct FontManager {
    pub font_map: HashMap<char, u8>,
    bind_group_layout : wgpu::BindGroupLayout,
    bind_group : wgpu::BindGroup
}


impl FontManager {
    pub async fn new(device : &wgpu::Device, queue : &wgpu::Queue) -> Result<wgpu::Texture, wgpu::SurfaceError>{

        let font = include_bytes!("../../assets/font/plp.otf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        let render_character_array = vec![
            'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','u','s','t','u','v','w','x','y','z',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            '0','1','2','3','4','5','6','7','8','9', ':'
        ];

        let mut max_size = [0, 0];
        let mut font_data = vec![];
        for character in render_character_array {
            let (metrics, bitmap) = font.rasterize(character, 30.0);
            font_data.push( (metrics, bitmap) );

            max_size[0] = max(max_size[0], metrics.width);
            max_size[1] = max(max_size[1], metrics.height);
        }



        //region Output Texture to Buffer (for output files )



        let u8_size = std::mem::size_of::<u8>() as u32;
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("font rendering command encoder") });
        let output_buffer_size = (u8_size* 256 * 256) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
                    |wgpu::BufferUsages::COPY_SRC
                    // this tells wpgu that we want to read this buffer from the cpu
                    // |wgpu::BufferUsages::MAP_READ
                    ,

            label: Some("font atlas buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);


        let char_in_row = 256/ max_size[0];
        for (index, font_datum) in font_data.iter().enumerate() {
            let metrics = font_datum.0;
            let bitmap = &font_datum.1;

            let size = wgpu::Extent3d {
                width:metrics.width as u32,
                height: metrics.height as u32,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("single-font texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::COPY_SRC |wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                bitmap,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(size.width),
                    rows_per_image: Some(size.height),
                },
                size,
            );


            let offset =(
                index % char_in_row * max_size[0] +
                        index / char_in_row * 256 * max_size[1] + 256 * (max_size[1] - size.height as usize)
            ) as wgpu::BufferAddress;

            encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &output_buffer,
                    layout: wgpu::ImageDataLayout {
                        offset,
                        bytes_per_row: Some(256),
                        rows_per_image: Some(256),
                    },
                },
                size,
            );

        }





        //region [ Make Font Atlas Texture ]
        let size = wgpu::Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("font_atlas"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC |wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });




        // //결국 이걸 바꿔야하네
        // //버퍼를 텍스쳐에 밀어넣지 말고 직접 그려줘야지...
        // encoder.copy_buffer_to_texture(
        //     wgpu::ImageCopyBuffer {
        //         buffer: &output_buffer,
        //         layout: wgpu::ImageDataLayout {
        //             offset :0,
        //             bytes_per_row: Some(256),
        //             rows_per_image: None,
        //         },
        //     },
        //     wgpu::ImageCopyTexture {
        //         texture: &texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //         aspect: Default::default(),
        //     },
        //     size);
        // output_buffer.unmap();
        queue.submit(Some(encoder.finish()));
        //endregion

        //region [ Save Font Atlas to png for test ]
        // We need to scope the mapping variables so that we can
        // {
        //     let buffer_slice = output_buffer.slice(..);
        //
        //     // NOTE: We have to create the mapping THEN device.poll() before await
        //     // the future. Otherwise the application will freeze.
        //     let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        //     buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        //         tx.send(result).unwrap();
        //     });
        //     device.poll(wgpu::Maintain::Wait);
        //     rx.receive().await.unwrap().unwrap();
        //
        //     let data = buffer_slice.get_mapped_range();
        //
        //     use image::{ImageBuffer, Luma};
        //     let buffer =
        //             ImageBuffer::<Luma<u8>, _>::from_raw(256, 256, data).unwrap();
        //     buffer.save("image2.png").unwrap();
        // }
        // output_buffer.unmap();
        //endregion



        Ok(texture)
    }

}