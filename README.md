# Digital Identity & Item Tagging on Stellar Soroban

**Thành viên nhóm:**  
- Nguyễn Lê Trường  
- Trần Kiều Linh  
- Nguyễn Anh Thơ

## Mô tả dự án

Dự án này xây dựng một hệ thống định danh số và quản lý sản phẩm cho thương mại điện tử dựa trên blockchain Stellar Soroban.

- **Định danh số (Digital Identity):** Người dùng có thể tạo định danh cá nhân bằng public key và metadata (ví dụ: tên, email, hash thông tin).
- **Đăng ký sản phẩm:** Cho phép nhập thông tin sản phẩm (ID, chủ sở hữu, loại, giá, metadata, hình ảnh) và sinh mã định danh duy nhất (product tag) cho mỗi sản phẩm.
- **Lưu trữ:** Dữ liệu sản phẩm được lưu tạm thời trên trình duyệt (localStorage). Định danh và sản phẩm có thể ghi lên smart contract Soroban nếu tích hợp ký và gửi transaction.
- **Tra cứu:** Giao diện cho phép tìm kiếm, xem chi tiết sản phẩm, xuất dữ liệu CSV.
- **Giao diện:** Thiết kế hiện đại, responsive, dễ sử dụng, sử dụng Astro + JS + Soroban client.

## Tính năng chính

- Tạo định danh người dùng trên blockchain.
- Đăng ký sản phẩm với thông tin chi tiết và sinh mã tag duy nhất.
- Tra cứu định danh và sản phẩm.
- Quản lý, tìm kiếm, xem chi tiết sản phẩm đã đăng ký.
- Xuất dữ liệu sản phẩm ra file CSV.
- Giao diện đẹp, dễ sử dụng, hỗ trợ mobile.

## Công nghệ sử dụng

- **Astro**: xây dựng giao diện web.
- **Soroban SDK**: viết smart contract (Rust).
- **@stellar/soroban-client**: kết nối và chuẩn bị giao dịch với Soroban.
- **LocalStorage**: lưu dữ liệu sản phẩm tạm thời trên trình duyệt.

## Hướng dẫn sử dụng

1. Chạy dự án Astro (`npm run dev` hoặc `yarn dev`).
2. Truy cập trang chủ để tạo định danh và đăng ký sản phẩm.
3. Nhấn "View Products" để xem, tìm kiếm, và xem chi tiết sản phẩm.
4. Để ghi dữ liệu lên blockchain, cần cấu hình contract ID và tích hợp ký/gửi transaction.

